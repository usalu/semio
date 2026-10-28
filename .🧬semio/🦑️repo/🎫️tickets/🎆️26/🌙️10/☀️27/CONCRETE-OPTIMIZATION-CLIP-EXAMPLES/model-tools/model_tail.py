def _sample(wwr, overhang):
    """📍 One sample point with its evaluated consequences."""
    study, _ = _frozen()
    return {"wwr": wwr, "overhang": overhang, **evaluate({**study["surface"]["design"], "wwr": wwr, "overhang": overhang})}


def levels():
    """🪜 Grid levels per design axis: wwr and overhang of the 3 × 3 grid, four bearings, three insulation thicknesses."""
    study, _ = _frozen()
    return copy.deepcopy(study["sampling"]["levels"])


def lhs_strata():
    """🧱 Stratum edges of the Latin hypercube on both axes and the stratum index pair of every point."""
    study, _ = _frozen()
    permutation = study["sampling"]["latin_permutation"]
    count = len(permutation)
    edges = lambda low, high: [low + index * (high - low) / count for index in range(count + 1)]
    return {"count": count, "wwr": edges(*study["surface"]["wwr"]), "overhang": edges(*study["surface"]["overhang"]),
            "index": [[index, stratum] for index, stratum in enumerate(permutation)]}


@_memo
def _samples(kind):
    """🎲 Equal-budget grid, recorded random and Latin-hypercube samples of the surface."""
    study, _ = _frozen()
    surface_study, sampling = study["surface"], study["sampling"]
    (wlow, whigh), (olow, ohigh) = surface_study["wwr"], surface_study["overhang"]
    if kind == "grid":
        points = [(*point, None) for point in itertools.product(sampling["levels"]["wwr"], sampling["levels"]["overhang"])]
    elif kind == "random":
        points = [(wlow + u * (whigh - wlow), olow + v * (ohigh - olow), None) for u, v in sampling["random"]]
    elif kind == "lhs":
        strata = lhs_strata()
        points = [((strata["wwr"][i] + strata["wwr"][i + 1]) / 2, (strata["overhang"][j] + strata["overhang"][j + 1]) / 2, [i, j])
                  for i, j in strata["index"]]
    else:
        raise ValueError("Unknown sampling kind")
    return [{**_sample(wwr, overhang), **({} if index is None else {"strata_index": index})} for wwr, overhang, index in points]


def samples(kind):
    """🎲 Return grid, random or lhs samples of the south wall surface with their consequences."""
    return _samples(kind)


def sampling_diagnostics(kind="random"):
    """🔍 Largest empty box and closest pair of one sample set, measured on axes normalized to their ranges."""
    study, _ = _frozen()
    (wlow, whigh), (olow, ohigh) = study["surface"]["wwr"], study["surface"]["overhang"]
    points = [((item["wwr"] - wlow) / (whigh - wlow), (item["overhang"] - olow) / (ohigh - olow)) for item in samples(kind)]
    best = (0.0, 0.0, 1.0, 0.0, 1.0)
    for left, right in itertools.combinations(sorted({0.0, 1.0, *(u for u, _ in points)}), 2):
        inside = sorted({0.0, 1.0, *(v for u, v in points if left < u < right)})
        gap, bottom = max((high - low, low) for low, high in itertools.pairwise(inside))
        area = (right - left) * gap
        if area > best[0]:
            best = (area, left, right, bottom, bottom + gap)
    pairs = [(math.dist(points[i], points[j]), i, j) for i, j in itertools.combinations(range(len(points)), 2)]
    distance, i, j = min(pairs)
    box = {"wwr": [wlow + best[1] * (whigh - wlow), wlow + best[2] * (whigh - wlow)],
           "overhang": [olow + best[3] * (ohigh - olow), olow + best[4] * (ohigh - olow)]}
    return {"largest_empty_box": box, "area": best[0], "closest_pair": [i, j, distance],
            "closest_delta": {"wwr": abs(points[i][0] - points[j][0]) * (whigh - wlow), "overhang": abs(points[i][1] - points[j][1]) * (ohigh - olow)}}


def _contrast(responses, signs):
    """➗ Mean of the high-level runs minus mean of the low-level runs."""
    return sum(sign * value for sign, value in zip(signs, responses, strict=True)) / (len(responses) / 2)


@_memo
def _factorial():
    """🧪 Exact contrasts of a two-level full factorial on one response."""
    study, _ = _frozen()
    setup = study["factorial"]
    base = _design(setup["design"])
    (a_id, a_levels), (b_id, b_levels) = ((factor["id"], factor["levels"]) for factor in setup["factors"])
    runs = []
    for number, (a, b) in enumerate(itertools.product((-1, 1), repeat=2), 1):
        design = {**base, a_id: a_levels[(a + 1) // 2], b_id: b_levels[(b + 1) // 2]}
        result = evaluate(design)
        runs.append({"id": f"r{number}", "coded": [a, b], "design": design, "response": result[setup["response"]], **result})
    y = [run["response"] for run in runs]
    effects = {"a": _contrast(y, [a for a, _ in (run["coded"] for run in runs)]), "b": _contrast(y, [run["coded"][1] for run in runs]),
               "ab": _contrast(y, [run["coded"][0] * run["coded"][1] for run in runs])}
    coefficients = [sum(y) / 4, effects["a"] / 2, effects["b"] / 2, effects["ab"] / 2]
    centre = {**base, a_id: sum(a_levels) / 2, b_id: sum(b_levels) / 2}
    actual = evaluate(centre)[setup["response"]]
    half = {**base, a_id: a_levels[0], b_id: (b_levels[0] + b_levels[1]) / 2}
    return {"factors": setup["factors"], "response": setup["response"], "runs": runs, "effects": effects, "coefficients": coefficients,
            "centre": {"design": centre, "predicted": coefficients[0], "actual": actual, "curvature": (actual - coefficients[0]) / coefficients[0],
                       "result": evaluate(centre)},
            "half_overhang": {"design": half, "response": evaluate(half)[setup["response"]]}}


def factorial():
    """🧪 Four runs r1…r4 with full results, effects (a, b, ab), coefficients (effects / 2) and the centre check."""
    return _factorial()


@_memo
def _fractional():
    """🧩 Half fraction 2^(3-1) with C = AB, compared with the true effects of the full 2³ plan."""
    study, _ = _frozen()
    setup = study["factorial"]
    base = _design(setup["design"])
    factors = [*setup["factors"], setup["third"]]
    response = lambda codes: evaluate({**base, **{factor["id"]: factor["levels"][(code + 1) // 2] for factor, code in zip(factors, codes, strict=True)}})[setup["response"]]
    full = [(codes, response(codes)) for codes in itertools.product((-1, 1), repeat=3)]
    runs = []
    for a, b in itertools.product((-1, 1), repeat=2):
        codes = [a, b, a * b]
        runs.append({"coded": codes, "design": {**base, **{factor["id"]: factor["levels"][(code + 1) // 2] for factor, code in zip(factors, codes, strict=True)}},
                     "response": response(codes)})
    y = [run["response"] for run in runs]
    true = {"ab": _contrast([value for _, value in full], [a * b for (a, b, _), _ in full]),
            "c": _contrast([value for _, value in full], [c for (_, _, c), _ in full])}
    return {"generator": "C=AB", "factors": factors, "runs": runs, "alias_contrast": _contrast(y, [run["coded"][2] for run in runs]),
            "true": {**true, "sum": true["ab"] + true["c"]}, "full": [{"coded": list(codes), "response": value} for codes, value in full]}


def fractional():
    """🧩 Four runs of the half fraction, the confounded C = AB contrast and the true C and AB effects."""
    return _fractional()


@_memo
def _variants(extra):
    """🏠 Named façade variants with all evaluated criteria."""
    study, _ = _frozen()
    return [{"id": item["id"], "title": item["title"], "design": _design(item["design"]), **({"extra": True} if item.get("extra") else {}), **evaluate(item["design"])}
            for item in study["variants"] if extra or not item.get("extra")]


def variants(extra=False):
    """🏠 The authored façade variants evaluated by the model; extra=True adds the dearer sixth one."""
    return _variants(bool(extra))


def criteria():
    """📏 Criteria with direction, unit, bilingual title and the model key they read."""
    study, _ = _frozen()
    return copy.deepcopy(study["criteria"])


def reweigh(weights, criterion, share):
    """🎚️ Give one criterion a share of 1 and scale the others in their old proportions."""
    others = sum(value for key, value in weights.items() if key != criterion)
    if criterion not in weights or not 0 <= share <= 1 or others <= 0:
        raise ValueError("Unknown criterion, share outside 0…1 or no other weight left")
    return {key: share if key == criterion else value / others * (1 - share) for key, value in weights.items()}


def index(weights, ids=None):
    """⚖️ Min–max normalize each criterion over the compared variants (0 best … 1 worst) and rank by the lowest weighted score."""
    items = criteria()
    rows = [row for row in variants(extra=True) if (ids is None and not row.get("extra")) or (ids is not None and row["id"] in ids)]
    identifiers = [item["id"] for item in items]
    if (not isinstance(weights, dict) or set(weights) != set(identifiers)
            or any(not math.isfinite(weights[key]) or weights[key] < 0 for key in identifiers) or sum(weights.values()) <= 0 or not rows):
        raise ValueError("Weights need every criterion id, finite and nonnegative, with a positive sum, and at least one variant")
    total = sum(weights.values())
    scores = [{"id": row["id"], "raw": {item["id"]: row[item["key"]] for item in items}, "normalized": {}} for row in rows]
    for item in items:
        column = [score["raw"][item["id"]] for score in scores]
        low, high = min(column), max(column)
        for score, value in zip(scores, column, strict=True):
            bad = 0.0 if high == low else (value - low) / (high - low)
            score["normalized"][item["id"]] = 1 - bad if item["direction"] == "max" else bad
    for score in scores:
        score["score"] = sum(weights[key] * score["normalized"][key] for key in identifiers) / total
    for rank, position in enumerate(sorted(range(len(scores)), key=lambda position: scores[position]["score"]), 1):
        scores[position]["rank"] = rank
    return scores


def _dominates(a, b, keys):
    """🥊 Whether a is at least as good as b in every key and strictly better in one (minimization)."""
    return all(a[key] <= b[key] for key in keys) and any(a[key] < b[key] for key in keys)


@_memo
def _pareto():
    """🏆 Cost versus electricity for a design grid, its front, payback steps and knees."""
    study, values = _frozen()
    setup, economics = study["pareto"], values["economics"]
    base = _design(setup["design"])
    designs = []
    for wwr, insulation, overhang in itertools.product(setup["wwr"], setup["insulation"], setup["overhang"]):
        design = {**base, "wwr": wwr, "insulation": insulation, "overhang": overhang}
        result = evaluate(design)
        designs.append({"id": f"w{round(wwr * 100)}-t{round(insulation * 100)}-d{round(overhang * 100)}", "design": design,
                        **{key: result[key] for key in ("cost", "electricity", "heating", "cooling", "lighting", "daylight")}})
    for item in designs:
        item["antidaylight"] = -item["daylight"]
    objectives = ("cost", "electricity")
    front = sorted((item for item in designs if not any(_dominates(other, item, objectives) for other in designs)), key=lambda item: (item["cost"], item["electricity"]))
    front3 = [item["id"] for item in designs if not any(_dominates(other, item, (*objectives, "antidaylight")) for other in designs)]
    for item in designs:
        del item["antidaylight"]
    lookup = {item["id"]: item for item in designs}
    steps = []
    for start, end in itertools.pairwise(front):
        saved = start["electricity"] - end["electricity"]
        ratio = (end["cost"] - start["cost"]) / saved
        steps.append({"from": start["id"], "to": end["id"], "cost": end["cost"] - start["cost"], "electricity": -saved, "saved": saved,
                      "ratio": ratio, "payback": ratio / economics["tariff"], "payback_hi": ratio / economics["tariff_hi"]})
    knee = {}
    for name, key in (("tariff", "payback"), ("tariff_hi", "payback_hi")):
        affordable = [step["to"] for step in steps if step[key] <= economics["service_life"]]
        knee[name] = affordable[-1] if affordable else front[0]["id"]
    refine = setup["refinement"]
    refined = evaluate({**base, **refine["design"]})
    candidate = {"cost": refined["cost"], "electricity": refined["electricity"]}
    refinement = {"member": refine["member"], "design": {**base, **refine["design"]}, "cost": refined["cost"], "electricity": refined["electricity"],
                  "dominates": [item["id"] for item in front if _dominates(candidate, item, objectives)]}
    revived = [lookup[identifier] for identifier in front3 if identifier not in {item["id"] for item in front}]
    third = max(revived, key=lambda item: item["daylight"])
    dominators = [item for item in front if _dominates(item, third, objectives)]
    third_example = {"dominated_2d": third["id"], "dominator": min(dominators, key=lambda item: item["cost"])["id"]}
    start, end = (lookup[setup["step"][key]] for key in ("from", "to"))
    return {"designs": designs, "front": [item["id"] for item in front], "dominated_example": setup["dominated_example"],
            "step": {"from": start["id"], "to": end["id"], "cost": end["cost"] - start["cost"], "electricity": end["electricity"] - start["electricity"]},
            "steps": steps, "tariff": economics["tariff"], "tariff_hi": economics["tariff_hi"], "service_life": economics["service_life"], "knee": knee,
            "refinement": refinement, "third_example": third_example, "front3": front3,
            "picks": [{"weight": weight, "id": weighted_pick(weight)} for weight in setup["weights"]]}


def pareto():
    """🏆 Evaluated design grid, front ids by cost, dominated pair, steps with payback, knees, refinement and third criterion."""
    return _pareto()


def weighted_pick(weight):
    """🎯 Front id minimizing the weighted sum of min–max normalized cost and electricity; weight is the share of electricity."""
    if not 0 <= weight <= 1:
        raise ValueError("The electricity weight must lie between 0 and 1")
    study, _ = _frozen()
    setup = study["pareto"]
    base = _design(setup["design"])
    designs = {item["id"]: item for item in _pareto_designs()}
    front = [designs[identifier] for identifier in _front_ids()]
    ranges = {key: (min(item[key] for item in front), max(item[key] for item in front)) for key in ("cost", "electricity")}
    normal = lambda item, key: (item[key] - ranges[key][0]) / (ranges[key][1] - ranges[key][0])
    return min(front, key=lambda item: (1 - weight) * normal(item, "cost") + weight * normal(item, "electricity"))["id"]


def _clip(value, bounds):
    """✂️ Keep a value inside its closed interval."""
    return min(bounds[1], max(bounds[0], value))


def _setup():
    """🛠️ Shared base design, bounds and ranges of the algorithm surface."""
    study, _ = _frozen()
    bounds = {"wwr": study["surface"]["wwr"], "overhang": study["surface"]["overhang"]}
    return study["algorithms"], _design(study["surface"]["design"]), bounds, {key: bounds[key][1] - bounds[key][0] for key in bounds}


def _evaluated(point, base):
    """🧾 A design dictionary of one algorithm point with its electricity."""
    return {"design": {**base, **point}, "electricity": _objective(point["wwr"], point["overhang"], base)}


@_memo
def _gradient_path():
    """📐 Numerical-gradient descent: probe four neighbours, step downhill in range-normalized coordinates."""
    setup, base, bounds, span = _setup()
    settings = setup["gradient"]
    point, previous, evaluations, path = dict(setup["start"]), None, 0, []
    for _ in range(settings["iterations"]):
        probes, gradient = [], {}
        for key in ("wwr", "overhang"):
            h = settings["difference"] * span[key]
            low, high = _clip(point[key] - h, bounds[key]), _clip(point[key] + h, bounds[key])
            pair = [_evaluated({**point, key: value}, base) for value in (low, high)]
            probes += [{"direction": f"{key}{sign}", **item} for sign, item in zip("-+", pair, strict=True)]
            gradient[key] = (pair[1]["electricity"] - pair[0]["electricity"]) / (high - low)
        evaluations += 5
        path.append({**_evaluated(point, base), "gradient": gradient, "slope": {"wwr": gradient["wwr"] / 100, "overhang": gradient["overhang"]},
                     "step": {key: 0.0 if previous is None else point[key] - previous[key] for key in point},
                     "probes": probes, "runs": 5, "evaluations": evaluations})
        previous = point
        point = {key: _clip(point[key] - settings["learning_rate"] * gradient[key] * span[key] ** 2, bounds[key]) for key in point}
    return path


def gradient_path():
    """📐 Iterates of fixed-rate descent: design, electricity, gradient (per unit), slope (per %-point and per m), step, probes, runs, evaluations."""
    return _gradient_path()


@_memo
def _pattern_path():
    """🧭 Compass search that polls four neighbours and halves the step when none improves."""
    setup, base, bounds, _ = _setup()
    settings = setup["pattern"]
    point, step = dict(setup["start"]), dict(settings["step"])
    current = _evaluated(point, base)
    path = [{**current, "step": dict(step), "probes": [], "moved": False, "evaluations": 1}]
    evaluations = 1
    for _ in range(settings["iterations"]):
        probes = [{"direction": f"{key}{'+' if sign > 0 else '-'}", **_evaluated({**point, key: _clip(point[key] + sign * step[key], bounds[key])}, base)}
                  for key, sign in itertools.product(("wwr", "overhang"), (1, -1))]
        best = min(probes, key=lambda probe: probe["electricity"])
        improved = best["electricity"] < current["electricity"] - settings["tolerance"]
        used, evaluations = dict(step), evaluations + len(probes)
        if improved:
            point, current = {key: best["design"][key] for key in point}, {"design": best["design"], "electricity": best["electricity"]}
        else:
            step = {key: step[key] * settings["contraction"] for key in step}
        path.append({**current, "step": used, "probes": probes, "moved": improved, "evaluations": evaluations})
    return path


def pattern_path():
    """🧭 Iterates of compass search with probes, the step used in each round, moved flag and cumulative evaluations."""
    return _pattern_path()


@_memo
def _genetic():
    """🧬 Elitist genetic algorithm with tournament selection, gene-wise crossover and mutation."""
    setup, base, bounds, _ = _setup()
    settings = setup["genetic"]
    lcg = settings["lcg"]
    state = lcg["seed"]

    def draw():
        nonlocal state
        state = (lcg["multiplier"] * state + lcg["increment"]) % lcg["modulus"]
        return state / lcg["modulus"]

    def individual(identifier, genes):
        return {"id": identifier, **_evaluated(genes, base)}

    population = [individual(f"g0-{index}", {key: bounds[key][0] + draw() * (bounds[key][1] - bounds[key][0]) for key in bounds})
                  for index in range(1, settings["population"] + 1)]
    generations, evaluations = [], len(population)
    for generation in range(settings["generations"] + 1):
        entry = {"population": population, "parents": [], "children": [], "tournaments": [], "elites": [], "survivors": [],
                 "best": min(population, key=lambda item: item["electricity"]), "evaluations": evaluations}
        generations.append(entry)
        if generation == settings["generations"]:
            break
        elite = sorted(population, key=lambda item: item["electricity"])[:settings["elite"]]

        def tournament():
            contenders = [population[int(draw() * len(population))] for _ in range(settings["tournament"])]
            winner = min(contenders, key=lambda item: item["electricity"])
            entry["tournaments"].append([*(item["id"] for item in contenders), winner["id"]])
            return winner
        for number in range(1, settings["population"] - settings["elite"] + 1):
            first = tournament()
            second = tournament()
            while second["id"] == first["id"]:
                second = tournament()
            donors = (first, second) if draw() < .5 else (second, first)
            inherited = {"wwr": donors[0]["design"]["wwr"], "overhang": donors[1]["design"]["overhang"]}
            genes, mutation = {}, {}
            for key in ("wwr", "overhang"):
                chance, size = draw(), draw()
                shift = (2 * size - 1) * settings["mutation"][key] if chance < settings["mutation"]["probability"] else 0.0
                genes[key] = _clip(inherited[key] + shift, bounds[key])
                mutation[key] = genes[key] - inherited[key]
            child = individual(f"g{generation + 1}-{number}", genes)
            entry["parents"].append([first["id"], second["id"]])
            entry["children"].append({"id": child["id"], "from": [first["id"], second["id"]],
                                      "genes": {"wwr": donors[0]["id"], "overhang": donors[1]["id"]},
                                      "mutation": mutation, "design": child["design"], "electricity": child["electricity"]})
        evaluations += len(entry["children"])
        population = [*elite, *({"id": child["id"], "design": child["design"], "electricity": child["electricity"]} for child in entry["children"])]
        entry["elites"] = [item["id"] for item in elite]
        entry["survivors"] = [item["id"] for item in population]
    return {"generations": generations, "lcg": lcg}


def genetic():
    """🧬 Generations: population, parents, tournaments [a, b, winner], children with genes and mutation, elites, survivors, best, evaluations."""
    return _genetic()


def convergence():
    """📈 Best-so-far electricity against simulation runs for gradient, pattern and genetic search on a shared budget."""
    study, _ = _frozen()
    budget = study["algorithms"]["budget"]
    orders = {
        "gradient": [value for item in _gradient_path() for value in (item["electricity"], *(probe["electricity"] for probe in item["probes"]))],
        "pattern": [_gradient_path()[0]["electricity"] if False else _pattern_path()[0]["electricity"],
                    *(probe["electricity"] for item in _pattern_path()[1:] for probe in item["probes"])],
        "genetic": [item["electricity"] for item in _genetic()["generations"][0]["population"]]
                   + [child["electricity"] for entry in _genetic()["generations"] for child in entry["children"]],
    }
    series = {}
    for method, values in orders.items():
        best, rows = math.inf, []
        for runs, value in enumerate(values[:budget], 1):
            best = min(best, value)
            rows.append([runs, best])
        series[method] = rows
    optimum = surface()
    return {"budget": budget, "map_minimum": optimum["minimum"]["electricity"], "runs_map": len(optimum["wwr"]) * len(optimum["overhang"]), "series": series}
