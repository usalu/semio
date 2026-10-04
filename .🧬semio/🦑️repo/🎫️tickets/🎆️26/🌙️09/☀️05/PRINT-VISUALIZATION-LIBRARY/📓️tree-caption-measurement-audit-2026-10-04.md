# Current Tree Caption Measurement Audit — 2026-10-04

Read-only inspected live hierarchy margin helper and caption/box emitters; no source edits or runtime execution.

Current node-margin measurement uses pgfinterruptpicture, globally stores its hbox with normalfont/scriptsize and authored text, then ends interruption before reading dimensions. This matches the established guide measurement interruption/global-box pattern. It explicitly avoids measuring under the surrounding TikZ nullfont. Actual standalone hierarchy caption and box emitters select scriptsize. Caption uses inner sep0 and directional1.2mm shift; helper allocates1.2mm plus measured total text height or width on that side and measured half width/height on perpendicular sides. Box helper uses text dimensions plus1.2mm padding, matching its existing0.6mm inner sep. Mark stroke half-width is separately admitted.

No immediate actual-font/size mismatch demonstrated for default family captions in this scope. normalfont measurement and scriptsize emitted text both use the normal document family in the inspected defaults; no authored alternative node-font route was found in the relevant family helper. Green3 runtime and actual final tree page validation remain with native owner. This source pass cannot certify layout or the disappearance of the previously viewed overflow.
