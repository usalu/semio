# E1 Capsule Dream Generator Report

**Agent:** E1  
**Ticket:** `26/08/09/PUZZLE-DESIGN-PARITY`  
**Goal:** `R26-02`  
**Scope:** ticket-local generator only (no puzzle plugin source edits)

## Deliverables

| Path | Role |
|------|------|
| `e1_capsule_dream_generator.py` | One-off compose → puzzle DSL / golden emitter |
| `🌙️capsule-dream-out/🗣️dream.5d.dsl.semio` | Full puzzle 5d DSL |
| `🌙️capsule-dream-out/🗣️dream.3d.dsl.semio` | Puzzle 3d projection |
| `🌙️capsule-dream-out/🗣️dream.2d.dsl.semio` | Puzzle 2d projection |
| `🌙️capsule-dream-out/🏅golden-poses.json` | Flat golden poses keyed by dream `pieceId` |
| `🌙️capsule-dream-out/📊e1-generator-stats.json` | Machine-readable run stats |

### Output sizes

| File | Bytes |
|------|------|
| `dream.2d.json` | 2,103,200 |
| `dream.3d.json` | 2,793,080 |
| `🏅golden-poses.json` | 990,237 |
| `📊e1-generator-stats.json` | 488 |
| `🗣️dream.2d.dsl.semio` | 2,038,176 |
| `🗣️dream.3d.dsl.semio` | 2,154,124 |
| `🗣️dream.5d.dsl.semio` | 2,844,327 |

## Inputs

- `compose/fixture/kit/dev/metabolism/wip/initialKit/kit.compose.json`
- `compose/fixture/kit/dev/metabolism/wip/initialKit/index.compose.json`
- `compose/fixture/kit/dev/metabolism/wip/initialKit/design/capsule-dream.design.compose.json`
- `compose/fixture/kit/dev/metabolism/wip/initialKit/design/flat.design.compose.json`
- Type shards under `.../initialKit/type/*.type.compose.json`
- Ports from kit typology `compatiblePorts`

## Mapping (normative)

| Compose | Puzzle 5d | Notes |
|---------|-----------|-------|
| Type | PartKind catalog row | 34 types used by dream |
| Port | GripKind catalog row | 15 ports used; `compatible_with` ← `compatiblePorts` |
| Connector | GripTemplate on kind + grip on part | `point` / `direction` / `t` / `mandatory` |
| Piece | Part | `anchor=fixed` iff pose present else `derived` |
| Connection | Fastener | `u→x`, `v→y`; full 8 params |
| Representation file | `/mesh/<filename>` | Primary part mesh prefers non-collider `.glb` |

Style reference: nakagin 5d `🗣️tower.dsl.semio`.

## Verification

```text
python3 e1_capsule_dream_generator.py
→ [DEBUG] e1 capsule-dream generator OK
```

### Counts (asserted in generator)

| Artifact | Expected | Actual |
|----------|----------|--------|
| 5d parts | 2880 | **2880** |
| 5d fasteners | 2864 | **2864** |
| 3d objects | 2880 | **2880** |
| 3d attractions | 2864 | **2864** |
| 2d nodes | 2880 | **2880** |
| 2d edges | 2864 | **2864** |
| Golden poses | 2880 | **2880** (0 missing) |
| Part kinds used | — | 34 |
| Grip kinds used | — | 15 |
| Fixed / Derived | — | 16 / 2864 |

### Spot checks

- All **2864** fastener endpoints use `pieceId:connectorId` (UUID:UUID).
- Sample fastener keeps compose transforms (`rotation=270`, `x=-1.2`, `y=1.2`).
- All **2880** parts have a primary `/mesh/…glb` URL.
- Golden keys = dream piece ids; `center.x/y` mapped from flat `u/v`; plane axes copied; join by piece **name** (flat regenerates ids).
- Kind catalogs filled with representations + grip templates (not empty stubs).

## Behaviour notes

1. Only 16 dream pieces store poses → Fixed; the other 2864 are Derived for flatten.
2. Flat design piece ids differ from dream; name equality is exact (2880/2880).
3. Nameless compose connectors get DSL `name=link` but keep UUID ids for targeting.
4. 3d/2d files are projections for E2/E3 install, nakagin-shaped.
5. Pre-existing `dream.2d.json` / `dream.3d.json` in the out folder were **not** produced by this generator and were left alone.

## Stats

```json
{
  "input_pieces": 2880,
  "input_connections": 2864,
  "used_types": 34,
  "used_ports": 15,
  "5d": {
    "parts": 2880,
    "fasteners": 2864,
    "part_kinds": 34,
    "grip_kinds": 15,
    "fixed": 16,
    "derived": 2864,
    "bytes": 2844327
  },
  "3d": {
    "objects": 2880,
    "attractions": 2864,
    "bytes": 2154124
  },
  "2d": {
    "nodes": 2880,
    "edges": 2864,
    "bytes": 2038176
  },
  "golden": {
    "poses": 2880,
    "missing": 0,
    "bytes": 990237
  }
}
```

## Handoff

- `🌙️capsule-dream-out/🗣️dream.5d.dsl.semio` → 5d example assets (E4)
- `🌙️capsule-dream-out/🗣️dream.3d.dsl.semio` → 3d example assets (E3)
- `🌙️capsule-dream-out/🗣️dream.2d.dsl.semio` → 2d example assets (E2)
- `🌙️capsule-dream-out/🏅golden-poses.json` → Wave 5 parity harness

E1 did not modify puzzle plugin sources.
