# Content Worker Brief (shared by all CV-* and NW-* workers)

Repo /home/deck/semio; follow /home/deck/semio/AGENTS.md strictly (no modifying git commands, no worktrees, `repo` binary for tools,
`[DEBUG] ` prefix for temporary logs, bilingual en/de natural texts). Others edit other asset folders and the engine at the same time.

1. Read T/references-physiology.md (which physiology numbers are verified — tag the rest authored-estimate), T/authoring-v2.md (binding), T/phase2-plan.md, S/🧬️schema/🟦️.ts (v2 types with docs), T/authoring.md (v1 rules still valid for
   rig/poses/clips/masses/contacts/loads/provenance), and for converting assets also the v1 notes in T/content-*.md and T/refine-*.md.
2. Work ONLY in your assigned asset folders `S/🗂️catalog/<emoji><category>/<asset-id>/🔣️.json` (create folders for new assets using the
   category folder names that exist: 👤️humans 🐕️animals 🌳️vegetation 🛋️furniture 🪑️objects 🚜️vehicles 🏗️machinery 🧱️components).
   Scratch scripts: T/🧰️scripts/<worker-id>/; generated previews: T/🗑️generated/<worker-id>/ (delete raw outputs at the end except
   evidence images you cite).
3. Order of work per asset:
   a. Physical scenario v2 (no tools needed): thermal regions with layers/geometry/heat capacity/body/leaf/operating/conditioned,
      `thermal.body` physiology, `thermal.respiration` with patterns + nostril/mouth emitters + chest motion, `plume` emitter for bodies,
      operating keys for machines, transpiration emitters with distributed anchors, exhaustive pose `thermal` bindings, clip tracks;
      remove v1-only fields (`lod`, `geometry`, `coreTempC`, `riseK`, `surfaceTempC`, `insulationM2KW`, `metabolicShare` at region top level,
      breath `periodS/duty`, inhale/exhale events). Sources for every parameter.
   b. Representations "500" "200" "100" "50" (+ symbols where useful): individually designed geometry per scale bound to the rig, pens,
      hatches, decisions, validRange — or explicit `unavailable` with reason/limiting feature/alternatives. For converted assets the old
      1:50 detail is raw material for the 1:50 drawing only; redraw the smaller scales as composed forms. Fix known v1 defects listed in
      the v1 notes (ground penetration, contact drift, etc.).
   c. Check and look: once T/core-v2-status.md reports the check/preview CLIs ready, run check (must print ✓), render every scale at print
      size and zoomed, every view (7) for the default pose and one clip instant, and READ the PNGs; iterate until each scale reads well at
      its printed size and passes the default print profile. Until the tools are ready, do (a) for all your assets and draft (b).
4. Report into T/content-v2-<worker-id>.md: per asset the representation decisions summary, unavailable scales with reasons, symbols,
   respiration/thermal parameter sources, check output, preview images inspected, remaining doubts. Final message: same, short.
