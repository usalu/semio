"""🧱️ Independent plane-stress CST reference using scikit-fem and SciPy only in tests."""

import json
import numpy as np
from scipy.linalg import eigh, eigvalsh
from scipy.spatial import Delaunay
from scipy.sparse import csr_matrix
import skfem
from skfem.helpers import ddot, sym_grad, trace, eye, grad


def reference(document):
    """🔮️ Assemble, solve and recover the portable coarse continuum without calling the subject."""
    region = document["regions"][0]
    material = next(m for m in document["materials"] if m["id"] == region["materialId"])
    points = np.array([[node["x"], node["y"]] for node in document["nodes"]])
    triangles = Delaunay(points).simplices
    for hole in region["holes"]:
        lower, upper = np.min(hole, axis=0), np.max(hole, axis=0)
        centers = np.mean(points[triangles], axis=1)
        triangles = triangles[~np.all((centers > lower) & (centers < upper), axis=1)]
    mesh = skfem.MeshTri(points.T, triangles.T)
    basis = skfem.Basis(mesh, skfem.ElementVector(skfem.ElementTriP1()))
    e, nu, thickness = material["e"], material["nu"], region["thickness"]
    mu, lam = e / (2.0 * (1.0 + nu)), e * nu / (1.0 - nu**2)

    def sigma(w):
        return 2.0 * mu * sym_grad(w) + lam * eye(trace(sym_grad(w)), 2)

    @skfem.BilinearForm
    def stiffness(u, v, w):
        return thickness * ddot(sigma(u), sym_grad(v))

    @skfem.BilinearForm
    def mass(u, v, w):
        return material["rho"] * thickness * np.einsum("i...,i...->...", u, v)

    k, m = stiffness.assemble(basis).toarray(), mass.assemble(basis).toarray()
    ids = {node["id"]: i for i, node in enumerate(document["nodes"])}
    for member in document["elements"]:
        a, b = ids[member["start"]], ids[member["end"]]
        delta = points[b] - points[a]
        length = np.linalg.norm(delta)
        direction = delta / length
        section = next(s for s in document["sections"] if s["id"] == member["sectionId"])
        vector = np.concatenate([-direction, direction])
        slots = np.concatenate([basis.nodal_dofs[:, a], basis.nodal_dofs[:, b]])
        k[np.ix_(slots, slots)] += e * section["area"] / length * np.outer(vector, vector)
        scalar = material["rho"] * section["area"] * length / 6.0
        m[np.ix_(slots, slots)] += scalar * np.kron([[2.0, 1.0], [1.0, 2.0]], np.eye(2))
    fixed = sorted(set(basis.nodal_dofs[["Tx", "Ty"].index(dof), ids[s["nodeId"]]] for s in document["supports"] for dof in s["fixed"]))
    free = np.setdiff1d(np.arange(basis.N), fixed)
    answers = {}
    for case in document["loadCases"]:
        pressure = sum(load["pressure"] for load in case["loads"] if load["kind"] == "area")
        body = pressure + (material["rho"] * thickness * 9.81 if case["selfWeight"] else 0.0)

        @skfem.LinearForm
        def area_force(v, w):
            return -body * v[1]

        f = area_force.assemble(basis)
        for load in case["loads"]:
            if load["kind"] == "nodal":
                f[basis.nodal_dofs[["Tx", "Ty"].index(load["dof"]), ids[load["nodeId"]]]] += load["value"]
        if case["selfWeight"]:
            for member in document["elements"]:
                a, b = ids[member["start"]], ids[member["end"]]
                section = next(s for s in document["sections"] if s["id"] == member["sectionId"])
                weight = material["rho"] * section["area"] * np.linalg.norm(points[b] - points[a]) * 9.81 / 2.0
                f[basis.nodal_dofs[1, [a, b]]] -= weight
        u = skfem.solve(*skfem.condense(csr_matrix(k), f, D=np.array(fixed)))
        stress = sigma(basis.interpolate(u))
        recovered = np.array([stress[0, 0].mean(), stress[1, 1].mean(), stress[0, 1].mean()])
        reactions = k @ u - f
        answer = {"displacements": u[basis.nodal_dofs].T.tolist(), "reactions": reactions[fixed].tolist(), "stress": recovered.tolist(), "stressMin": np.min(stress[[0, 1, 0], [0, 1, 1]], axis=(1, 2)).tolist(), "stressMax": np.max(stress[[0, 1, 0], [0, 1, 1]], axis=(1, 2)).tolist()}
        if not region["holes"]:

            @skfem.BilinearForm
            def geometric(u, v, w):
                return thickness * np.einsum("ij...,ki...,kj...->...", stress, grad(u), grad(v))

            kg = geometric.assemble(basis).toarray()
            diagonal = max(float(np.max(np.abs(np.diag(kg)[free]))), 1e-12)
            b = -kg[np.ix_(free, free)] + np.eye(len(free)) * diagonal * 1e-6
            answer["factors"] = eigvalsh(k[np.ix_(free, free)], b)[:document["analysis"]["bucklingCount"]].tolist()
        answers[case["id"]] = answer
    frequencies = np.sqrt(eigh(k[np.ix_(free, free)], m[np.ix_(free, free)])[0]) / (2.0 * np.pi)
    return {"cases": answers, "frequenciesHz": [] if region["holes"] else frequencies[:document["analysis"]["modalCount"]].tolist()}


def fixture(ctx, needle):
    """🧫️ Read one declared portable fixture URI."""
    uri = next(token for step in ctx.scenario["steps"] for token in step["text"].split() if token.startswith("shared://") and needle in token)
    return json.loads(ctx.input_bytes(uri).decode("utf-8"))


def projected(answer, scales):
    """📏️ Normalize the three independently computed physical response scales."""
    result = {"cases": {}, "frequenciesHz": [f / scales["frequency"] for f in answer["frequenciesHz"]]}
    for key, case in answer["cases"].items():
        result["cases"][key] = {"displacements": (np.array(case["displacements"]) / scales["translation"]).tolist(), "reactions": (np.array(case["reactions"]) / scales["force"]).tolist(), "stress": (np.array(case["stress"]) / scales["stress"]).tolist()}
        for field in ("stressMin", "stressMax"):
            result["cases"][key][field] = (np.array(case[field]) / scales["stress"]).tolist()
        if "factors" in case:
            result["cases"][key]["factors"] = [f / scales["factor"] for f in case["factors"]]
    return result


def handler(name):
    """⚖️ Solve in the oracle role and independently verify the manufactured hole stress field."""
    def run(ctx):
        from semio_repo_test import Outcome
        document = fixture(ctx, name)
        answer = reference(document)
        expected = fixture(ctx, "expected.results")[name]
        if document["regions"][0]["holes"]:
            np.testing.assert_allclose(answer["cases"]["patch"]["stress"], [-10000.0, -20000.0, 0.0], atol=1e-7, rtol=1e-10)
        projection = projected(answer, expected["scales"])
        held = projected(expected["answer"], expected["scales"])
        np.testing.assert_allclose(projection["frequenciesHz"], held["frequenciesHz"], atol=1e-10, rtol=1e-10)
        for case, fields in projection["cases"].items():
            for field, values in fields.items():
                np.testing.assert_allclose(values, held["cases"][case][field], atol=1e-10, rtol=1e-10)
        return Outcome(projection, raw=json.dumps(answer, separators=(",", ":"), ensure_ascii=False).encode("utf-8"))
    return run


def adapter():
    """🧭️ Register only independent numerical reference handlers."""
    from semio_repo_test import Adapter
    built = Adapter("python")
    for suffix, name in [("triangle", "🔺️triangle.snapshot.json"), ("mixed", "🌉️mixed.snapshot.json"), ("hole", "🕳️hole.snapshot.json"), ("scaled-hole", "📏️scaled-hole.snapshot.json")]:
        built = built.oracle("continuum-" + suffix, handler(name))
    return built
