# Scene Interop Review

Reviewed the new production RefreshMoving source after its six permanent tests passed. The module is a pure owned mixin and imports no third-party types. It calls the original frame update, obtains the current family, and selects drawable members outside the original static family. If the renderer has no saved static image it renders the current family; if the moving family becomes empty it invalidates the saved image before rendering the current family. This addresses detached-child ghosts and the independently reproduced translucent double-compositing boundary case.

It does not register a scene updater, so it preserves the original static-Wait decision. It does not alter tracker state, semantic vectors, installed Manim, or the frozen snapshot/keyed/Sprite helpers. Six representative non-target fingerprints matched before and after creation. Actual native morph, image/vector, glyph-length, widget and Wait tests support the behavior; complete course renders and final-source verification remain required.

Animation-suspended updaters during concurrent FadeIn and tracker motion require explicit reveal-then-ramp timelines. The affected scene clients were reviewed and their changed-state frames compared to fresh Cairo before freeze; the interop module does not claim to override that animation lifecycle.
