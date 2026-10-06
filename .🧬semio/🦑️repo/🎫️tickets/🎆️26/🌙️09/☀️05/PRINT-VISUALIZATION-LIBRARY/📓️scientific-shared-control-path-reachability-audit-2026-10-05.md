# Scientific Shared Control Path Reachability Audit — 2026-10-05

Shared canvas controls can be globally read yet unused for an individual family. semio-viz-scientific-field.sty770 sci_enter only resets and installs defaults/authored keys; it does not call sci_frame. sci_frame477 is the only inspected shared title/xlabel/ylabel painter and controls frame/grid/ticks. sci_window_keys751 consumes authored domain/range. Registry semio-viz-family.sty38/55 installs and invokes family definitions without adding that shared frame.

Direct traced native families sci-bracket (geometry1135 and slot helper1160+), sci-staff (signal461 and note helper495+) and sci-mesh (geometry852 and node/cell helpers866+) never invoke sci_frame or window_keys. Their authored title, xlabel/ylabel and frame/grid/tick switches therefore have no shared painter effect. The source-owned schema nevertheless exposes title as Caption printed above the canvas, axes as Draw the frame and the tick labels, along with the other canvas descriptors. title/xlabel/ylabel need neutral actual glyph inclusion/placement controls; axes/grid/ticks need actual primitive-count/coordinate controls. Domain/range applicability should be schema-defined for structural raw-coordinate families instead of silently accepting a window no caller uses.

A precise default discrepancy also exists in these three current owners: schema axes defaulttrue (same generic shared descriptor) while each native family default list explicitly sets axes=false/grid=false. This is not repaired merely by documenting the shared helper reset default. Family overrides govern actual rendering. The source metadata pipeline must derive/declare actual family defaults and applicability, or the native owner must implement the advertised family contract consistently.

This finding is source behavior, not an executed compiler RED. The proposed correction must remain in existing owners; no external runtime library or parallel renderer is justified. It is separate from thirteen variables that have no reads anywhere: here a global read is not reachable from an installed family's actual call path.

Shared-path baseline sources additionally retained:
- semio-viz-scientific-field.sty: 3087D2FF69C4CB68DE6AD32B8B8AE8E7F8D5E23C13A9B503E23FD87F33FEB62F
- semio-viz-family.sty: D7C8BAB330D4BE95008D9BFEF75E01F2D3C1CD38AA9A1536A24D465272F01F53
