"""Turns the no-oracle decisions that claim mutation capabilities without any third-party reference into surveyed, justified gaps."""
import json, re
root = "/Users/ueli/Documents/semio/"
NATIVE = "is a format this repository defines (isSemioNativeArtifact), so no third party reads or writes its documents; every candidate below models a neighbouring object and cannot apply or adjudicate this vocabulary's mutations. The owner's recorded debt stands: a verified native second implementation, written from the schema and the committed vectors, is what would discharge it."
S = {
 "equation-mutation-semantics": ("✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/🏅️standards/🔖️1/🪆️subsets/🕸️graph/🔮️oracles/🔣️.json", ["equation-1-mutate"], {
   "ecosystemsSearched": ["pypi", "crates.io", "npm"],
   "candidatesConsidered": [
     {"package": "sympy", "ecosystem": "pypi", "verdict": "cannot-express-the-mutation", "reason": "A computer-algebra system over symbolic expressions; it has no model of this document's equation graph, its directed flag, layout algorithm and seed, or its point cloud, so change-coefficient on the document's AST and every graph and geometry kind have no counterpart."},
     {"package": "networkx", "ecosystem": "pypi", "verdict": "cannot-express-the-mutation", "reason": "Holds bare nodes and edges; it could redo connect/disconnect on an edge list but carries neither the per-graph algorithm/seed fields nor the document the mutation edits, so it adjudicates a different operation."},
     {"package": "shapely", "ecosystem": "pypi", "verdict": "cannot-express-the-mutation", "reason": "Geometries are immutable coordinate sequences with no per-point identity, so insert-point, remove-point and move-point by the document's point index have no counterpart."},
     {"package": "petgraph", "ecosystem": "crates.io", "verdict": "cannot-express-the-mutation", "reason": "A Rust graph container with the same limits as networkx: no algorithm field, no point geometry, no reader for this carrier."}],
   "whyNoneQualifies": "s.mathematical.equation " + NATIVE}),
 "sequence-step-graph-mutation-semantics": ("✏️s/🔌️plugins/🎬️sequence/🗿️artifacts/🎬️sequence/🏅️standards/🔖️1/🪆️subsets/✳️any/🔮️oracles/🔣️.json", ["sequence-1-mutate"], {
   "ecosystemsSearched": ["pypi", "npm", "crates.io"],
   "candidatesConsidered": [
     {"package": "networkx", "ecosystem": "pypi", "verdict": "cannot-express-the-mutation", "reason": "Models a DAG of edges only; the step position, the collapse flag and the document's edge records that move-step, change-step-collapsed, connect-steps and disconnect-steps write are not part of it."},
     {"package": "bpmn-js", "ecosystem": "npm", "verdict": "cannot-express-the-mutation", "reason": "Reads BPMN process diagrams, whose sequence-flow model has no counterpart of this document's steps, collapse state or parameter records."},
     {"package": "graphviz", "ecosystem": "pypi", "verdict": "cannot-express-the-mutation", "reason": "Lays out DOT graphs; it neither reads this carrier nor holds step positions or collapse flags as document state."}],
   "whyNoneQualifies": "s.sequence.sequence " + NATIVE}),
 "drawing-mutation-semantics": ("✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🔮️oracles/🔣️.json", ["drawing-1-mutate"], {
   "ecosystemsSearched": ["pypi", "npm", "crates.io"],
   "candidatesConsidered": [
     {"package": "quick-xml", "ecosystem": "crates.io", "verdict": "cannot-express-the-mutation", "reason": "Already registered for the kinds the SVG export carries; rename-layer, set-layer-locked and set-layer-blend-mode write fields that export never emits, so the reader has nothing to see."},
     {"package": "svgelements", "ecosystem": "pypi", "verdict": "cannot-express-the-mutation", "reason": "Parses SVG, which is the only third-party carrier of this document, and the three remaining kinds are not in that carrier."},
     {"package": "paper", "ecosystem": "npm", "verdict": "cannot-express-the-mutation", "reason": "A vector scene graph with its own layer model; it does not read this document, so a rename or lock applied there says nothing about this vocabulary."}],
   "whyNoneQualifies": "s.draw.drawing " + NATIVE}),
 "gis-gisterrain-config-mutation-semantics": ("✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🏔️gisterrain/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🎭️modes/👁️view/🪟️windows/🏔️terrain/🎚️config/🔮️oracles/🔣️.json", [], {
   "ecosystemsSearched": ["pypi", "npm", "crates.io"],
   "candidatesConsidered": [
     {"package": "pydeck", "ecosystem": "pypi", "verdict": "cannot-express-the-mutation", "reason": "Holds a deck.gl view state of its own, not this editor window's persisted camera record, so set-camera applied there is a different record."},
     {"package": "cesium", "ecosystem": "npm", "verdict": "cannot-express-the-mutation", "reason": "A globe renderer with an imperative camera; it neither reads nor writes GisTerrainWindowConfig."}],
   "whyNoneQualifies": "GisTerrainWindowConfig is one editor window's persisted-local camera state that this repository defines; no third party reads or writes it, so no reference can adjudicate set-camera."}),
 "raw-buffer-no-format": ("✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/💾️binary/🏅️standards/🔖️raw/🪆️subsets/✳️any/🔮️oracles/🔣️.json", [], {
   "ecosystemsSearched": ["pypi", "crates.io", "posix"],
   "candidatesConsidered": [
     {"package": "coreutils dd/truncate", "ecosystem": "posix", "verdict": "cannot-express-the-mutation", "reason": "Rewrites or truncates bytes at an offset, but has no notion of this vocabulary's contract: which offsets and remove lengths are refused, what an out-of-range splice returns, or the inverse each kind must yield."},
     {"package": "numpy", "ecosystem": "pypi", "verdict": "cannot-express-the-mutation", "reason": "Array insert/delete reproduce byte arithmetic only; the refusal and inverse semantics under test are this subset's own definitions."}],
   "whyNoneQualifies": "A raw buffer has no format, so the mutation vocabulary (offset/remove_len splice, append, truncate, snapshot) is itself the specification; a byte tool reproduces arithmetic but cannot adjudicate the refusals and inverses this subset defines."}),
 "os-config-opening-preferences-mutation-semantics": ("🧰️framework/🛍️products/💻️os/🎚️config/🔮️oracles/🔣️.json", [], {
   "ecosystemsSearched": ["pypi", "npm", "crates.io"],
   "candidatesConsidered": [{"package": "xdg-mime", "ecosystem": "posix", "verdict": "cannot-express-the-mutation", "reason": "Maps MIME types to desktop files; this record maps semio artifact dialect triples and roles to semio app references, which it cannot hold."}],
   "whyNoneQualifies": "os.config.opening is this operating system's own preference record keyed by semio dialects and apps; no third party implements it, so none can adjudicate set-default-app or clear-default-app."}),
 "os-config-merge-policy-mutation-semantics": ("🧰️framework/🛍️products/💻️os/🎚️config/🔮️oracles/🔣️.json", [], {
   "ecosystemsSearched": ["pypi", "npm", "crates.io"],
   "candidatesConsidered": [{"package": "automerge", "ecosystem": "npm", "verdict": "cannot-express-the-mutation", "reason": "A CRDT library with its own merge semantics; it has no record of which merge policy this operating system's authority applies, which is the only thing change-merge-policy edits."}],
   "whyNoneQualifies": "os.config.merge-policy is repository-owned authority configuration; no third party holds or edits it, so none can adjudicate change-merge-policy."}),
 "os-config-identity-mutation-semantics": ("🧰️framework/🛍️products/💻️os/🎚️config/🔮️oracles/🔣️.json", [], {
   "ecosystemsSearched": ["pypi", "npm", "crates.io"],
   "candidatesConsidered": [{"package": "keyring", "ecosystem": "pypi", "verdict": "cannot-express-the-mutation", "reason": "Stores credentials in the platform keychain; it has no model of this operating system's IdentitySetting session record that sign-in and sign-out establish or clear."}],
   "whyNoneQualifies": "os.config.identity is repository-owned session configuration (Option<Identity>); no third party implements it, so none can adjudicate sign-in or sign-out."}),
}
files = {}
for did, (path, drop, survey) in S.items():
    d = files.setdefault(path, json.load(open(root + path, encoding="utf-8")))
    dec = next(x for x in d["noOracleDecisions"] if x["id"] == did)
    dec["capabilities"] = [c for c in dec["capabilities"] if c not in drop]
    dec["coversMutations"] = True
    dec["referenceSurvey"] = survey
for path, d in files.items():
    raw = open(root + path, encoding="utf-8").read()
    indent = len(re.match(r"\{\n( *)", raw).group(1))
    open(root + path, "w", encoding="utf-8").write(json.dumps(d, ensure_ascii=False, indent=indent) + "\n")
    print(path.split("/")[-3], "written")
