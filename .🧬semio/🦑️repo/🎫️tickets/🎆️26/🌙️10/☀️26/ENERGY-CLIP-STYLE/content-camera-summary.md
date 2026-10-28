# Transition-Only Virtual UI

The virtual UI appears only during opening and closing transitions. After the real topic and video clicks, a 0.6-second camera move zooms uniformly into the measured source-image area. Playback then fills the output with no navbar, footer, window border or restore chip. At the end, the camera reverses before the native cursor restores the video and topic overviews. The source content and its own authored graphics remain unchanged.

The three-second UI interaction route remains intact in each direction; the camera adds 0.6 seconds to each transition before frame-rate quantization. Source frames, source audio timing, right-side fused controls and common overview boundaries are preserved. Snapshot endpoints and body share one frame-rate/fit normalization. Decoded displayed frames, rather than copied MP4 packets, determine the finite body and audio duration. Obsolete shell compositing and its API were removed.

[Updated preview](./content-camera.mp4) · [Playback frame](./content-camera-body.png) · [Zoom frame](./content-camera-zoom.png) · [Return frame](./content-camera-return.png). This review uses a captured physical excerpt; details are in [preview inputs](./content-camera-preview.md).

Verification passed: nine contract checks; eight hierarchy checks; native browser camera and representative encoded-media checks; English/German fused restore; silent video, frame-rate conversion and edited-MP4 displayed-frame regressions; and both registered native interaction timing tests. The retained real preview has 105 frames at 1280 × 720 and 8 fps. Its 47 playback frames match the supplied input exactly (mean RGB error 0), camera joins differ by less than one RGB level on average, and overview boundaries are pixel-identical.

See [tests](./content-camera-tests.md), [implementation](./content-camera-implementation.md), [independent media verification](./content-camera-verification.md) and [visual audit](./content-camera-audit.md).
