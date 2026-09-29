#!/usr/bin/env python3
"""🗂️ T14 T3 set `kind-choices` (lands WITH W4's `w4-kind-spec-identity.py`, same train): a creation picker offers exactly the
DECLARED artifact kinds of a package, each labelled with that kind's own en + de label.

Every editor app is labelled "Editor" (SDK default), so a picker labelled by app showed "Editor" for every kind (S18/G12 on 7800).
The TS resolver `artifactKindChoices` (`🛂️manifest/🟦️.ts`) and its Rust twin `artifact_kind_choices` now resolve an app's kind
by schema among every kind its PACKAGE declares (a viewer declares none — it shares its editor's), skip an app whose schema
names no declared kind (a per-user Home, a studio: not creatable), and never fall back to the app label. Needs W4's identity
set first (9 packages' kind schemas ≠ their app DOCUMENT_SCHEMA would otherwise vanish from the picker). Law (TS, in-source
vitest): declared kind labels, viewers share their editor's kind, no undeclared kind, no two choices share a label.

usage: kind-choices.py --dry-run | --write | --revert [--root <tree>]   (backups: .🧬semio/🌐hub/s14-t14-land/kind-choices/{live,scratch}/)"""
import sys
from pathlib import Path

ROOT = Path(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else Path("/Users/ueli/Documents/semio")
BACKUP = Path("/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-t14-land/kind-choices") / ("live" if ROOT == Path("/Users/ueli/Documents/semio") else "scratch")
TS = "🧰️framework/🔨️modules/🛂️manifest/🟦️.ts"
TS_TEST = "🧰️framework/🔨️modules/🛂️manifest/🧪️tests/🧪️hostresolvedargs/🟦️.ts"
RS = "🧰️framework/🔨️modules/🛂️manifest/🦀️.rs"
EDITS = [
    (TS, """ * contributes one choice per dialect coordinate, labelled with the KIND's own label (the app's, then
 * the manifest's `artifactKinds` entry of that schema) — never its editor app's label, which every kind
 * of a package would share. Deduped by dialect coordinate (first manifest/app wins — callers pass owner
 * manifests first so the owner's label wins over a later contributor's), sorted by coordinate for
 * determinism — the pure resolver behind `ActionArgControl.artifactKind`. */""",
     """ * and whose package declares an artifact kind of that schema (on any of its apps — a viewer shares its
 * editor's kind — or on the manifest) contributes one choice per dialect coordinate, labelled with that
 * KIND's own label, never its app's label (every editor app is labelled "Editor"). An app whose schema
 * names no declared kind (a per-user Home, a studio) is not a creatable kind. Deduped by dialect
 * coordinate (first manifest/app wins — callers pass owner manifests first so the owner's label wins
 * over a later contributor's), sorted by coordinate for determinism — the pure resolver behind
 * `ActionArgControl.artifactKind`. */"""),
    (TS, """      const kinds = [...(app.artifactKinds ?? []), ...(manifest.artifactKinds ?? [])] as readonly { readonly schema?: string; readonly label?: unknown }[];
      const kind = kinds.find((candidate) => candidate.schema === app.io.artifactSchema);
      byCoordinate.set(coordinate, { kindId: app.dialect.artifactKind, schema: app.io.artifactSchema, dialect: app.dialect, label: resolveNativeLabel(kind === undefined ? app.label : kind.label) });""",
     """      const kinds = [...manifest.apps.flatMap((other) => (other as { readonly artifactKinds?: readonly unknown[] }).artifactKinds ?? []), ...(manifest.artifactKinds ?? [])] as readonly { readonly schema?: string; readonly label?: unknown }[];
      const kind = kinds.find((candidate) => candidate.schema === app.io.artifactSchema);
      if (kind === undefined) continue;
      byCoordinate.set(coordinate, { kindId: app.dialect.artifactKind, schema: app.io.artifactSchema, dialect: app.dialect, label: resolveNativeLabel(kind.label) });"""),
    (TS_TEST, """        apps: apps.map((app) => ({ role: app.role, dialect: app.dialect, label: { native: app.label ?? { en: app.dialect.artifactKind, de: app.dialect.artifactKind } }, io: { artifactSchema: app.artifactSchema } })),""",
     """        apps: apps.map((app) => {
          const label = { native: app.label ?? { en: app.dialect.artifactKind, de: app.dialect.artifactKind } };
          return { role: app.role, dialect: app.dialect, label, io: { artifactSchema: app.artifactSchema }, artifactKinds: app.artifactSchema === "" ? [] : [{ schema: app.artifactSchema, label }] };
        }),"""),
    (TS_TEST, """    it("artifactKindChoices labels every choice with its kind's own label, never the shared editor app label", () => {""",
     """    it("artifactKindChoices labels every choice with its package's declared kind label (viewers share their editor's kind), never the shared app label, and offers no undeclared kind", () => {"""),
    (TS_TEST, """            { role: "editor", dialect: { artifactKind: "s.wfc.3d", standard: "1", subset: "*" }, label: editor, io: { artifactSchema: "wfc.3d" } },
          ],""",
     """            { role: "editor", dialect: { artifactKind: "s.wfc.3d", standard: "1", subset: "*" }, label: editor, io: { artifactSchema: "wfc.3d" } },
            { role: "viewer", dialect: { artifactKind: "s.wfc.2d", standard: "1", subset: "*" }, label: editor, io: { artifactSchema: "wfc.2d" } },
            { role: "editor", dialect: { artifactKind: "s.wfc.home", standard: "1", subset: "*" }, label: editor, io: { artifactSchema: "wfc.home" } },
          ],"""),
    (TS_TEST, """      expect(new Set(choices.map((choice) => choice.label.de)).size).toBe(choices.length);
    });""",
     """      expect(new Set(choices.map((choice) => choice.label.de)).size).toBe(choices.length);
      expect(artifactKindChoices(manifests, ["viewer"]).map((choice) => choice.label)).toEqual([{ en: "2D", de: "2D" }]);
      expect(choices.map((choice) => choice.kindId)).not.toContain("s.wfc.home");
    });"""),
    (RS, """/// 🗂️ Every artifact-kind choice for the given `roles`: every app across `manifests` whose `role` is
/// in `roles` and whose `io.artifact_schema` is non-empty contributes one choice per dialect
/// coordinate. Deduped by dialect coordinate (first manifest/app wins — callers pass owner manifests
/// first so the owner's label wins over a later contributor's), sorted by coordinate for determinism
/// — the pure resolver behind `ActionArgControl::ArtifactKind`.""",
     """/// 🗂️ Every artifact-kind choice for the given `roles` — Rust twin of TS `artifactKindChoices` (law in
/// `🧪️tests/🧪️hostresolvedargs/🟦️.ts`): every app across `manifests` whose `role` is in `roles`, whose
/// `io.artifact_schema` is non-empty and whose package declares a kind of that schema (on any of its apps
/// — a viewer shares its editor's kind — or on the manifest) contributes one choice per dialect coordinate,
/// labelled with that kind's own label, never its app's; an undeclared schema is not a creatable kind.
/// Deduped by dialect coordinate (first manifest/app wins — callers pass owner manifests first so the
/// owner's label wins over a later contributor's), sorted by coordinate for determinism."""),
    (RS, """            let label = app.artifact_kinds.iter().chain(manifest.artifact_kinds.iter()).find(|kind| kind.schema == app.io.artifact_schema).map_or_else(|| app.label.clone(), |kind| kind.label.clone());
            by_coordinate.entry(app.dialect.to_coordinate()).or_insert_with(|| ArtifactKindChoice { kind_id: app.dialect.artifact_kind.clone(), schema: app.io.artifact_schema.clone(), dialect: app.dialect.clone(), label });""",
     """            let Some(kind) = manifest.apps.iter().flat_map(|other| other.artifact_kinds.iter()).chain(manifest.artifact_kinds.iter()).find(|kind| kind.schema == app.io.artifact_schema) else {
                continue;
            };
            by_coordinate.entry(app.dialect.to_coordinate()).or_insert_with(|| ArtifactKindChoice { kind_id: app.dialect.artifact_kind.clone(), schema: app.io.artifact_schema.clone(), dialect: app.dialect.clone(), label: kind.label.clone() });"""),
]
mode = sys.argv[1] if len(sys.argv) > 1 else "--dry-run"
texts = {rel: (ROOT / rel).read_text(encoding="utf-8") for rel in {rel for rel, _, _ in EDITS}}
pending = applied = bad = 0
for rel, old, new in EDITS:
    text = texts[rel]
    if mode == "--revert":
        if text.count(new) == 1:
            texts[rel] = text.replace(new, old); applied += 1
        continue
    if text.count(new) == 1 and text.count(old) == 0:
        applied += 1; print("applied  ", rel, "::", old.strip().splitlines()[0][:80])
    elif text.count(old) == 1:
        pending += 1; texts[rel] = text.replace(old, new); print("pending  ", rel, "::", old.strip().splitlines()[0][:80])
    else:
        bad += 1; print("MISSING  ", rel, "::", old.strip().splitlines()[0][:80])
if mode in ("--write", "--revert") and not bad:
    for rel, text in texts.items():
        path = ROOT / rel
        if text != path.read_text(encoding="utf-8"):
            if mode == "--write":
                (BACKUP / rel).parent.mkdir(parents=True, exist_ok=True)
                (BACKUP / rel).write_bytes(path.read_bytes())
            path.write_text(text, encoding="utf-8")
print(f"{len(EDITS)} edits, {pending} pending, {applied} {'reverted' if mode == '--revert' else 'applied'}, {bad} bad")
sys.exit(1 if bad else 0)
