# Renderer Viewport Ownership — Follow-up Audit Queue

The shared viewport implementation is native/JSON/TypeScript verified for both poses. Root's narrow read-only source check identifies the next concrete audit boundary; no renderer protocol migration is made here.

- The relocated UI scene protocol at `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🟦️.ts` still represents `World3dScene.cameraJson` as a serialized string. The native scene producer and renderer parse/gesture sides must be examined before deciding whether the wire string is intentional transport ownership or a missing typed protocol.
- That same UI scene source defines `NodeGraphViewport` with exactly readonly x/y/zoom fields. Compare its semantic requirements and all native/TypeScript consumers to shared `Viewport2d`; do not leave a redundant alias simply to preserve an old name.
- The React target at `🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🟦️.tsx` defines another `Camera` interface near line4289 and reexports a distinct external `ThreeCamera`. Determine which are domain-neutral pose, scene/capture data, renderer-native object, or tutorial keyframe before proposing shared ownership.
- IconRenderCamera and authored/tutorial cameras may have distinct semantics. Similar field names alone do not establish an ownership violation.

A Terra read-only agent will inventory actual shapes, call sites, source/schema/codecs, serialization boundaries, and exact tests once the UI protocol repair slot is free. Its repair recommendation should distinguish the app/domain camera defaults already preserved by Note, FEM, Remodel, and Drawing from the reusable pose structure itself.
