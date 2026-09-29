"""🔎 Validates the architecture quiz catalog and its quizzes: draft-07 schema (jsonschema) plus the semantic rules of design §2/§4/§6/§7.

Usage (repo root): .venv/Scripts/python.exe "<ticket>/validate_quiz_content.py" [output-file]
"""

import json
import math
import sys
import unicodedata
from importlib.metadata import version
from itertools import combinations
from pathlib import Path

import jsonschema
from referencing import Registry, Resource
from referencing.jsonschema import DRAFT7

ROOT = Path(__file__).resolve().parents[7]
SCHEMA_PATH = ROOT / "🧰️framework" / "🛍️products" / "❓️quiz" / "🧬️schema" / "🔣️.json"
CATALOG_PATH = ROOT / "🎓️teaching" / "🏛️architecture" / "❓️quiz" / "🔣️.json"
SLUG_CHARS = set("abcdefghijklmnopqrstuvwxyz0123456789-")
MIN_RATIO = 1.25

lines: list[str] = []
errors: list[str] = []
warnings: list[str] = []


def out(text: str = "") -> None:
    lines.append(text)


def error(where: str, text: str) -> None:
    errors.append(f"{where}: {text}")


def warn(where: str, text: str) -> None:
    warnings.append(f"{where}: {text}")


def load(path: Path) -> dict:
    return json.loads(path.read_text(encoding="utf-8"))


def check_emoji_path(path: Path) -> None:
    for part in path.relative_to(ROOT).parts:
        first = part[0]
        if unicodedata.category(first) == "So" and (len(part) < 2 or part[1] != "️"):
            error(str(path.relative_to(ROOT)), f"path segment {part!r} lacks U+FE0F after its emoji")


def check_texts(node, where: str) -> None:
    if isinstance(node, dict):
        if set(node.keys()) == {"en", "de"}:
            for language in ("en", "de"):
                if not node[language].strip():
                    error(where, f"empty {language} text")
            return
        for key, value in node.items():
            check_texts(value, f"{where}/{key}")
    elif isinstance(node, list):
        for index, value in enumerate(node):
            check_texts(value, f"{where}/{index}")


def unique(ids: list[str], where: str, scope: str) -> None:
    seen = set()
    for id_ in ids:
        if id_ in seen:
            error(where, f"duplicate {scope} id {id_!r}")
        seen.add(id_)


def check_draw(task: dict, where: str) -> None:
    if "draw" in task:
        if not 2 <= task["draw"] <= len(task["items"]):
            error(where, f"draw {task['draw']} outside [2, {len(task['items'])}]")


def ratio_report(values: list[tuple[str, float]], where: str, logarithmic: bool) -> str:
    ordered = sorted(values, key=lambda entry: entry[1])
    ratios = []
    for (left_id, left), (right_id, right) in zip(ordered, ordered[1:]):
        if right == left:
            warn(where, f"duplicate value {left} ({left_id}, {right_id})")
            ratios.append(1.0)
            continue
        ratio = right / left if logarithmic and left > 0 else math.inf
        ratios.append(ratio)
        if logarithmic and ratio < MIN_RATIO - 1e-9:
            warn(where, f"neighbours {left_id} {left} → {right_id} {right} differ by factor {ratio:.2f} < {MIN_RATIO}")
    shown = ", ".join(f"{value:g}" for _, value in ordered)
    smallest = min(ratios) if ratios else math.nan
    return f"{len(values)} values [{shown}]; smallest neighbour factor {smallest:.2f}"


def check_classification(task: dict, where: str) -> None:
    categories = [category["id"] for category in task["categories"]]
    unique(categories, where, "category")
    unique([item["id"] for item in task["items"]], where, "item")
    for item in task["items"]:
        if item["category"] not in categories:
            error(f"{where}/{item['id']}", f"unknown category {item['category']!r}")
        if "explanation" not in item:
            error(f"{where}/{item['id']}", "missing explanation")
    axes = task.get("axes", [])
    unique([axis["id"] for axis in axes], where, "axis")
    for axis in axes:
        if not axis["max"] > axis["min"]:
            error(f"{where}/axes/{axis['id']}", "max ≤ min")
    profiled = [category for category in task["categories"] if "profile" in category]
    if profiled and not axes:
        error(where, "profiles without axes")
    if profiled and len(profiled) != len(task["categories"]):
        warn(where, "only some categories carry profiles")
    for category in profiled:
        profile = category["profile"]
        for axis in axes:
            if axis["id"] not in profile:
                error(f"{where}/{category['id']}", f"profile lacks axis {axis['id']!r}")
            elif not axis["min"] <= profile[axis["id"]] <= axis["max"]:
                error(f"{where}/{category['id']}", f"{axis['id']} = {profile[axis['id']]} outside [{axis['min']}, {axis['max']}]")
        for key in profile:
            if key not in {axis["id"] for axis in axes}:
                error(f"{where}/{category['id']}", f"profile names unknown axis {key!r}")
    used = {item["category"] for item in task["items"]}
    unused = [category for category in categories if category not in used]
    counts = {category: sum(item["category"] == category for item in task["items"]) for category in categories}
    out(f"    categories: {', '.join(f'{category} ×{counts[category]}' for category in categories)}" + (f"; unused: {unused}" if unused else ""))
    if profiled:
        normalised = {
            category["id"]: [(category["profile"][axis["id"]] - axis["min"]) / (axis["max"] - axis["min"]) for axis in axes]
            for category in profiled
        }
        distances = {(left, right): math.dist(normalised[left], normalised[right]) for left, right in combinations([c["id"] for c in profiled], 2)}
        d_max = max(distances.values())
        closest = min(distances, key=distances.get)
        out(f"    profile distances: d_max = {d_max:.3f}; closest pair {closest} = {distances[closest]:.3f} → partial credit {max(0.0, 1 - distances[closest] / d_max):.2f}")


def check_sorting(task: dict, where: str) -> None:
    unique([item["id"] for item in task["items"]], where, "item")
    logarithmic = task["quantity"]["scale"] == "logarithmic"
    values = [(item["id"], item["value"]) for item in task["items"]]
    for item_id, value in values:
        if logarithmic and value <= 0:
            error(f"{where}/{item_id}", f"non-positive value {value} on a logarithmic scale")
    if len({value for _, value in values}) != len(values):
        error(where, "sorting values are not strictly distinct")
    for item in task["items"]:
        if "explanation" not in item:
            error(f"{where}/{item['id']}", "missing explanation")
    listed = [value for _, value in values]
    out(f"    listed ascending: {listed == sorted(listed)}; span {math.log10(max(listed) / min(listed)):.1f} decades" if logarithmic else f"    listed ascending: {listed == sorted(listed)}")
    out(f"    {ratio_report(values, where, logarithmic)}")


def check_matching(task: dict, where: str) -> None:
    dimensions = [dimension["id"] for dimension in task["dimensions"]]
    unique(dimensions, where, "dimension")
    unique([item["id"] for item in task["items"]], where, "item")
    for item in task["items"]:
        if set(item["values"]) != set(dimensions):
            error(f"{where}/{item['id']}", f"values {sorted(item['values'])} ≠ dimensions {dimensions}")
        if "explanation" not in item:
            error(f"{where}/{item['id']}", "missing explanation")
    for dimension in task["dimensions"]:
        logarithmic = dimension["quantity"]["scale"] == "logarithmic"
        values = [(item["id"], item["values"][dimension["id"]]) for item in task["items"] if dimension["id"] in item["values"]]
        for item_id, value in values:
            if logarithmic and value <= 0:
                error(f"{where}/{item_id}", f"non-positive {dimension['id']} {value} on a logarithmic scale")
        out(f"    {dimension['id']} [{dimension['quantity']['unit']}]: {ratio_report(values, f'{where}/{dimension['id']}', logarithmic)}")
    if len(dimensions) > 1:
        inversions = sum(1 for a, b in combinations(task["items"], 2) if (a["values"][dimensions[0]] - b["values"][dimensions[0]]) * (a["values"][dimensions[1]] - b["values"][dimensions[1]]) < 0)
        out(f"    cross-dimension inversions ({dimensions[0]} vs {dimensions[1]}): {inversions} of {len(task['items']) * (len(task['items']) - 1) // 2} pairs")


def main() -> None:
    schema = load(SCHEMA_PATH)
    registry = Registry().with_resource(schema["$id"], Resource.from_contents(schema, default_specification=DRAFT7))
    quiz_validator = jsonschema.Draft7Validator({"$ref": f"{schema['$id']}#/$defs/Quiz"}, registry=registry)
    catalog_validator = jsonschema.Draft7Validator({"$ref": f"{schema['$id']}#/$defs/Catalog"}, registry=registry)
    jsonschema.Draft7Validator.check_schema(schema)
    out(f"jsonschema {version('jsonschema')} — schema {SCHEMA_PATH.relative_to(ROOT)} is a valid draft-07 schema")

    catalog = load(CATALOG_PATH)
    check_emoji_path(CATALOG_PATH)
    schema_errors = sorted(catalog_validator.iter_errors(catalog), key=lambda e: list(e.absolute_path))
    out(f"\n## Catalog {CATALOG_PATH.relative_to(ROOT)}")
    out(f"  schema (Catalog): {'valid' if not schema_errors else f'{len(schema_errors)} errors'}")
    for problem in schema_errors:
        error("catalog", f"{list(problem.absolute_path)}: {problem.message}")
    check_texts(catalog, "catalog")
    if (CATALOG_PATH.parent / catalog["$schema"].split("#")[0]).resolve() != SCHEMA_PATH.resolve():
        error("catalog", "$schema does not resolve to the contract")
    paragraphs = catalog["introduction"]["paragraphs"]
    out(f"  introduction: {len(paragraphs)} paragraphs")
    if not 3 <= len(paragraphs) <= 5:
        error("catalog", "introduction must have 3–5 paragraphs")
    if len(set(catalog["quizzes"])) != len(catalog["quizzes"]):
        error("catalog", "duplicate quiz paths")

    quizzes = []
    for relative in catalog["quizzes"]:
        path = (CATALOG_PATH.parent / relative).resolve()
        if not path.exists():
            error("catalog", f"quiz path {relative} does not exist")
            continue
        check_emoji_path(path)
        quiz = load(path)
        quizzes.append(quiz)
        out(f"\n## Quiz {quiz['id']} — {path.relative_to(ROOT)}")
        problems = sorted(quiz_validator.iter_errors(quiz), key=lambda e: list(e.absolute_path))
        out(f"  schema (Quiz): {'valid' if not problems else f'{len(problems)} errors'}")
        for problem in problems:
            error(quiz["id"], f"{list(problem.absolute_path)}: {problem.message}")
        if (path.parent / quiz["$schema"].split("#")[0]).resolve() != SCHEMA_PATH.resolve():
            error(quiz["id"], "$schema does not resolve to the contract")
        check_texts(quiz, quiz["id"])
        unique([task["id"] for task in quiz["tasks"]], quiz["id"], "task")
        for task in quiz["tasks"]:
            where = f"{quiz['id']}/{task['id']}"
            out(f"  task {task['id']} ({task['kind']}): {len(task['items'])} items, draw {task.get('draw', '—')}")
            check_draw(task, where)
            {"classification": check_classification, "sorting": check_sorting, "matching": check_matching}[task["kind"]](task, where)

    unique([quiz["id"] for quiz in quizzes], "catalog", "quiz")
    unique([badge["id"] for badge in catalog["badges"]], "catalog", "badge")
    quiz_ids = {quiz["id"] for quiz in quizzes}
    out("\n## Badges")
    for badge in catalog["badges"]:
        rule = badge["rule"]
        if rule["kind"] == "perfect-quiz":
            ok = rule["quiz"] in quiz_ids
            selected = sum(len(quiz["tasks"]) for quiz in quizzes if quiz["id"] == rule["quiz"])
        elif rule["kind"] == "perfect-tasks":
            ok = "quiz" not in rule or rule["quiz"] in quiz_ids
            selected = sum(1 for quiz in quizzes for task in quiz["tasks"] if rule.get("taskKind", task["kind"]) == task["kind"] and rule.get("quiz", quiz["id"]) == quiz["id"])
        else:
            ok, selected = True, len(quizzes)
        if not ok:
            error(f"badge {badge['id']}", "references an unknown quiz")
        if selected == 0:
            error(f"badge {badge['id']}", "selects no task and can never be awarded")
        out(f"  {badge['emoji']} {badge['id']} ({rule['kind']}{', ' + rule.get('taskKind', '') if rule.get('taskKind') else ''}{', ' + rule['quiz'] if 'quiz' in rule else ''}): selects {selected} {'quizzes' if rule['kind'] == 'completed-quizzes' else 'tasks'}")

    out("\n## Result")
    out(f"  errors: {len(errors)}")
    lines.extend(f"    ✗ {entry}" for entry in errors)
    out(f"  warnings (neighbour factor < {MIN_RATIO} or duplicates): {len(warnings)}")
    lines.extend(f"    ! {entry}" for entry in warnings)
    text = "\n".join(lines) + "\n"
    sys.stdout.buffer.write(text.encode("utf-8"))
    if len(sys.argv) > 1:
        Path(sys.argv[1]).write_text(text, encoding="utf-8")
    sys.exit(1 if errors else 0)


if __name__ == "__main__":
    main()
