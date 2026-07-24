# Mode ⊗ orientation conjugation

## Request
Gizmo and projection pane must not repeat. Top/front/corners belong on the gizmo; ortho/axo/2pt/3pt/fisheye belong on the pane. Every setting must work with every other (e.g. top + fisheye).

## Model
`WorldProjectionSpec = { mode, orientation }`
- **mode** (pane): orthographic | axonometric(Iso/Di/Tri) | oblique(Cab/Cav/Mil) | one/two/three-point | curvilinear
- **orientation** (gizmo): cardinal (plan/top/…) | corner (NE… + hemisphere) | free (3D)

## Behaviour
- Gizmo hit → changes `orientation` only
- Pane/template → changes `mode` only (keeps current orientation)
- `computeWorldProjectionPose` looks from orientation, camera family from mode
- Template tree no longer lists Top/Front (gizmo owns those)

## Verify
- `vitest-compose-final2.txt` — infinite-world-r3f 136 passed
