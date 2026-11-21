# React tiled-map touch navigation

## Gesture behavior

The React map now applies one-finger movement to the map session's pan operation once the contact travels six CSS pixels from its origin. From then on, the gesture owns navigation through release, even when the finger returns to its starting point. A release inside the threshold picks the feature under the release point. The six-pixel threshold matches the existing map session.

The renderer follows the finger locally and publishes the final camera on release. Cancellation or loss of pointer capture commits the last received movement. A second contact cancels pending selection or ends the session pan, then gives the camera to the shared pinch recognizer. The map keeps contact capture through the handoff and publishes once after the last contact ends. The desktop primary-button marquee and middle-button pan continue through the original mouse paths.

## React verification

The mounted regression suite reproduced four failures on the original implementation. It uses a language-neutral touch fixture and Three.js vector math to check the camera displacement, plus mounted pointer events to cover feature picking, the terminal sample, returning to the start, pinch handoff, lost capture, and mouse input. All 51 tests passed across the four scoped suites covering map component behavior, touch and mouse gestures, gesture lifecycle, and cancellation. Vitest ran with the renderer's repository configuration and one worker after the repo tool command remained queued for machine capacity.

## WGPU parity

The native scene's shared touch recognizer previously returned single-contact events to the primary selection path. The WGPU work uses the same touch contract and can consume the neutral fixture. Device verification is tracked separately in this ticket.
