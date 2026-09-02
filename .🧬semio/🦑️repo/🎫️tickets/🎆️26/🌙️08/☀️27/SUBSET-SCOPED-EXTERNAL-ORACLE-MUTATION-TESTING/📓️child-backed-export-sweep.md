# 📤️ Child-Backed Export Sweep

## Contract

A child handle is persistence identity, never semantic export content. An exporter that needs child
content must project a materialized typed local owner. If the owner is absent or has the wrong exact
type, the conversion returns `IoError`; it must not serialize `{childId,target}`, invent an empty
scene, or weaken an otherwise complete carrier to `Lossy`.

`IoFidelity` describes successful conversions. A complete semantic projection remains `Exact` (or
`Canonical` when the target syntax normalizes representation); inability to acquire required input
is a failed conversion, not a lossy success.

The serializer boundary receives only `&Snapshot`. The registered `serializer_entry` first decodes
only the parent pack and supplies no store, child payload set, `ChildContentView`, or resolver.
`mathematical_graph_geometry_from_children` can reconstruct a scene from three already-resolved
child snapshots, but it cannot resolve the handles by itself. Rebuilding therefore belongs at a
host/composition boundary that owns those child snapshots, before the exporter is called. Until that
boundary exists, a wire-only parent must fail closed.

## The Two Affected Plugins

| Plugin | Exact/canonical carrier | Missing-owner result | Fidelity |
|---|---|---|---|
| `➗️mathematical` JSON | `{graph,geometry,equation}` | `IoError` | `Exact` |
| `➗️mathematical` Markdown | canonical JSON block containing the same fixture | `IoError` | `Canonical` |
| `➗️mathematical` CSV | node rows projected from the materialized graph | `IoError` | `Lossy` because edges, geometry, and equation are omitted |
| `🎬️sequence` JSON | `{schema,steps,edges}` | `IoError` | `Exact` |
| `🎬️sequence` Markdown | canonical JSON block containing the same fixture | `IoError` | `Canonical` |
| `🎬️sequence` CSV | step rows projected from the materialized scene | `IoError` | `Lossy` because edges are omitted |

Both text routes remain explicit unsupported errors and declare `Lossy`; they do not report a
successful empty export. `sequence_working_scene_for_handle` now returns `Option` and no longer
turns an absent owner into `SequenceWorkingScene::default()`.

## Fleet Sweep

The established 29-artifact inventory was checked against this contract:
`✒️writer`, `➗️mathematical`, `🧩️assembly`, `🌊️flow`, `🏔️gisterrain`, `🗺️gismap`,
`🎬️present`, `🎥️shooting`, `🎬️sequence`, `🏛️program`, `🧊️process3d`, `💠️lowpoly`,
`🔌️wires`, `📋️forms`, `📏️layout`, `📐️cad`, `📘️en1990`, `📙️din18599`, `📖️playbook`,
`📜️imperative`, `📸️remodel`, `🔋️model`, `🔌️jack`, `🕸️dag`, `🖨️raster`, `🧿️semio`,
`🖐️5d`, `🧊️3d`, and `🗂️curate`.

The direct-parent-serialization pattern remains a fleet issue outside the two plugins changed here.
It cannot be made correct centrally without changing the IO boundary: native persistence must keep
serializing `ArtifactChild` handles, while semantic foreign exports must not. Each remaining
artifact therefore needs a complete typed projection and the same required-owner precondition, or a
host-supplied resolved-child export input. Merely changing its fidelity would bless missing content
as a successful conversion and violates this contract.

The drawing/json/sequence-CSV corpora previously authored in this ticket remain outside this defect:
`SemioDrawingSnapshot` is inline, and only semio kit/object are child-backed.

