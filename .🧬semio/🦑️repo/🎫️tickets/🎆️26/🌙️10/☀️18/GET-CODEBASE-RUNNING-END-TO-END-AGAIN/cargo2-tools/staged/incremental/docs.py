#!/usr/bin/env python3
"""📝 Anchored, idempotent README edit of batch `incremental` (README is a hot file, so no whole-file hash): docs.py check|apply|revert [--root DIR]
Each pair is (old, new) and must occur exactly once in its state; check prints base|applied|drift per pair, apply/revert exit 2 on drift."""
import os, sys
REL = "🧰️framework/🛍️products/🦑️repo/README.md"
PAIRS = [
 ("the units whose source or dependency artifacts changed; the focused `-p` crate keeps rustc's incremental state in the agent's own lane | live |",
  "the units whose source or dependency artifacts changed; every workspace crate that misses the cache compiles with rustc's incremental state in the agent's own lane (so the next edit of it recompiles incrementally) and is still published, the focused `-p` crate always compiles there and is never published | live |"),
 ("a miss compiles once, publishes content-addressed objects and the entry last. Units of the packages selected with `-p` keep `-C incremental` in the lane and are never published.",
  "a miss compiles once with the command line Cargo gave, publishes content-addressed objects and the entry last; a workspace unit that Cargo compiles incrementally keeps its rustc session in the lane, so the next edit of that crate (also a dependency of the focused one) recompiles incrementally. Units of the packages selected with `-p` always compile with `-C incremental` in the lane and are never published."),
 ("die Einheiten, deren Quelle oder Abhängigkeits-Artefakte sich geändert haben; das mit `-p` gewählte Crate behält den inkrementellen Zustand von rustc in der eigenen Lane des Agenten | live |",
  "die Einheiten, deren Quelle oder Abhängigkeits-Artefakte sich geändert haben; jedes Workspace-Crate, das den Cache verfehlt, kompiliert mit dem inkrementellen Zustand von rustc in der eigenen Lane des Agenten (die nächste Änderung daran kompiliert also inkrementell) und wird trotzdem veröffentlicht, das mit `-p` gewählte Crate kompiliert immer dort und wird nie veröffentlicht | live |"),
 ("ein Fehltreffer kompiliert einmal, veröffentlicht inhaltsadressierte Objekte und zuletzt den Eintrag. Einheiten der mit `-p` gewählten Pakete behalten `-C incremental` in der Lane und werden nie veröffentlicht.",
  "ein Fehltreffer kompiliert einmal mit der Kommandozeile, die Cargo übergab, veröffentlicht inhaltsadressierte Objekte und zuletzt den Eintrag; eine Workspace-Einheit, die Cargo inkrementell kompiliert, behält ihre rustc-Sitzung in der Lane, sodass die nächste Änderung dieses Crates (auch einer Abhängigkeit des gewählten) inkrementell kompiliert. Einheiten der mit `-p` gewählten Pakete kompilieren immer mit `-C incremental` in der Lane und werden nie veröffentlicht."),
]
def main():
    verb = sys.argv[1] if len(sys.argv) > 1 else "check"
    root = os.path.abspath(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else None
    if root is None:
        root = os.path.dirname(os.path.abspath(__file__))
        while not os.path.isdir(os.path.join(root, "🧰️framework")) and root != "/": root = os.path.dirname(root)
    path = os.path.join(root, REL); text = open(path, encoding="utf8").read(); states = []
    for old, new in PAIRS: states.append("applied" if text.count(new) == 1 and text.count(old) == 0 else "base" if text.count(old) == 1 and text.count(new) == 0 else "drift")
    for state, (old, _) in zip(states, PAIRS): print(state, old[:70])
    if verb == "check": return 0 if "drift" not in states else 1
    if "drift" in states: return 2
    for old, new in PAIRS: text = text.replace(old, new) if verb == "apply" else text.replace(new, old)
    tmp = path + ".docs-stage"; open(tmp, "w", encoding="utf8").write(text); os.replace(tmp, path); print(verb, "done"); return 0
sys.exit(main())
