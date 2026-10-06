# 📓️ S5 stdio live journey — a declared parameter leaf in the history editor

Author: S5-TEXT-STDIO, 2026-10-05. Purpose: the second-artifact live proof for goal clause 5 ("every input of a mutation has
information about its UI element") and clause 12 ("artifact-agnostic"): the probe exercises puzzle 2d only, and none of the 1226
inputs declared in the stdio / trinity / writer / vcs leaves this session is reachable live. Nothing below was run live by me; every
fact is read from the tree (paths given) or printed by the framework reader.

## Subject

- Artifact: `s.stdio.pdf` standard `1.7` subset `base`, editor surface (crate `semio-s-artifact-stdio-pdf`, composition
  `🌎️hub/🧩️compositions/🗄️stdio`).
- Leaf: `set-page-rotation` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🧬️mutations/🔄️set-page-rotation/`).
- Producer in the editor: the page panel control `pdf-page-rotation`, label "Rotation" / "Drehung", bound to the editor action
  `set-page-rotation` with argument `x` (`…/🧱️base/✏️editor/🖼️page/🦀️.rs:253`, `:2746`); the action refuses anything but 0, 90, 180,
  270 (`:850`) and publishes ONE `PdfMutation::SetPageRotation { index, rotation }`.
- Needs: a describe + activation of the stdio composition (the descriptors travel in `INPUT_SCHEMAS`), any PDF with at least two
  pages as the document (the bundled pdf example of the editor is enough if it has two pages; otherwise insert one with the page
  panel first).

## The six steps (run in React AND wgpu, `en` then `de`)

1. **Open** a two-page PDF document in the stdio pdf editor. Expect: page 1 shown upright, History panel lists the load only.
2. **Rotate page 1 to 90°** with the page panel's "Rotation" / "Drehung" control. Expect: the page renders rotated; the history
   gains ONE row labelled "Set page 0 rotation to 90" / "Drehung von Seite 0 auf 90 setzen".
3. **Add a later edit** that depends on nothing (page panel: set the document language, or insert a text object on page 2).
   Expect: a second row after the rotation row.
4. **Open the rotation row for editing** (row action Edit). Expect: the shell enters history editing (band "History editing" /
   "Verlaufsbearbeitung"), the later row is shown as not yet applied, and the editor lists EXACTLY two input rows:
   - "Page" / "Seite" — a number **stepper**, value 0, minimum 0, step 1, no decimals, description "Index of the page, counted
     from 0." / "Index der Seite, ab 0 gezählt.";
   - "Rotation" / "Drehung" — a **dial**, value 90, unit `°`, range 0–270 (soft bounds), step 90, **four snap ticks at 0, 90, 180,
     270**, description "Clockwise rotation of the page when it is displayed, a multiple of 90°." / "Drehung der Seite im
     Uhrzeigersinn bei der Anzeige, ein Vielfaches von 90°.".
5. **Turn the dial to 180 and Accept.** Expect: the preview shows page 1 upside down while editing; after Accept the downstream
   row replays without warning or error and the final document shows page 1 at 180° with the later edit intact. Then **change
   "Page" to 5** (a page that does not exist) and Accept: the mutation itself must report a named, localized outcome on its row
   (target missing), the session stays repairable; set it back to 1 → page 2 is rotated instead and the replay is clean.
6. **Finalize** once as "new alternative" and once as "overwrite". Expect: the alternative lists both histories; after reload the
   chosen rotation persists and the History panel shows the edited row.

Withdraw-only counter-check (same session, 30 seconds): load a second file through "replace source" so the history gains a
`set-snapshot` row. Its row must offer **Withdraw / Restore only** — no Edit action, no editor with zero rows (descriptor
`"editable": false`, design §22.20).

## The descriptor the editor must show

Printed by the framework's own reader over the leaf schema on disk
(`bun T/🗑️generated/s5-text-stdio/descriptor.ts <leaf schema path>` → `mutationInputDefs`; the Rust twin `mutation_input_defs` is
what `TimeTravelStoreState::open` reads):

```json
[
  { "id": "/index",
    "label": { "en": "Page", "de": "Seite" },
    "description": { "en": "Index of the page, counted from 0.", "de": "Index der Seite, ab 0 gezählt." },
    "schema": { "kind": "number", "min": 0, "step": 1, "integer": true, "precision": 0 },
    "presentation": { "kind": "stepper" }, "required": true, "group": "target", "order": 10 },
  { "id": "/rotation",
    "label": { "en": "Rotation", "de": "Drehung" },
    "description": { "en": "Clockwise rotation of the page when it is displayed, a multiple of 90°.",
                     "de": "Drehung der Seite im Uhrzeigersinn bei der Anzeige, ein Vielfaches von 90°." },
    "schema": { "kind": "number", "min": 0, "step": 90, "integer": true, "unit": "°", "snaps": [0, 90, 180, 270],
                "softMin": 0, "softMax": 270, "precision": 0 },
    "presentation": { "kind": "dial" }, "required": true, "group": "value", "order": 20 }
]
```

## What a FAIL would mean (routing)

| Observation | Meaning | Owner |
|---|---|---|
| The row has no Edit action | the pdf composition was not re-described (stale `INPUT_SCHEMAS`) or the leaf lost its schema | coordinator (describe) / S5-TEXT-STDIO |
| A plain number field instead of the dial, or no ticks | the renderer drops `presentation: dial` or `snaps` for this artifact | S5-UI (React) / S5-WGPU |
| Labels in English under `de` | the editor reads the glossary instead of the declared label | S5-RUNTIME |
| "Page" = 5 faults the session instead of a row outcome | the pdf leaf's `diff` panics or returns an unnamed error for a missing page | S5-TEXT-STDIO |
| The `set-snapshot` row opens an editor | `editable: false` not read (derive / describe stale) | S5-GATES / coordinator |

## Other declared leaves worth one glance in the same activation

- gif `set-screen-size`: two `px` steppers (step 1) — integers with a unit.
- semio graph `create-node` (any composed graph child): `x` / `y` / `width` / `height` snap to the window's `gridFactor`
  (`snapSource: {config}`), ports with labelled direction options.
- semio brep `create-vertex.tol`: logarithmic slider 1e-9 … 1e-1 with decade snaps.
- zip `rename-entry`: a read-only reference chip for the entry and a text input for the new name (stdio editors publish no
  selection domain, so "Use selection" is disabled — a known limit, not a failure).
