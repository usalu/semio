# Apple Input and Fleet Availability Audit

2026-11-21. Read-only inspection; no physical device operated, installation, reservation, or native build attempted.

## Pinned WebKit

Installed Playwright 1.64.1 WebKit implements `RawTouchscreenImpl.tap` with `Input.dispatchTapEvent` and exposes no touch movement or multi-contact API. The Chromium CDP touch command in Playwright protocol types is not a WebKit session. Synthetic DOM touch events do not establish trusted device input. The WebKit raw keyboard implementation does send `Input.dispatchKeyEvent` for key down and up, including in mobile contexts; a feasible next test is to focus the map, observe `isTrusted` and key values, send `page.keyboard.press('ArrowRight')` and `page.keyboard.press('+')`, and assert a settled camera/pixel change. This is an emulated hardware-keyboard browser test, not physical iOS keyboard or touch proof. Its success is not established by this source inspection.

For trusted Apple pan and pinch, use Safari on a physical iPhone/iPad or a properly provisioned iOS Simulator with an input automation path capable of multi-contact gestures. Record device/OS/browser, trusted pointer samples, pre/post settled map imagery and camera state; verify network tile completion and gesture outcome. Simulator or browser emulation cannot establish physical-device performance. Do not claim a simulator gesture path works until demonstrated.

## Local availability and fleet

`xcode-select -p` reports `/Library/Developer/CommandLineTools`. `xcrun simctl list devices available` and `xcrun devicectl list devices` both report the utilities unavailable. `/Applications` lists no Xcode app; `adb` and `idevice_id` are absent from PATH. This is insufficient to establish that no network-attached device exists, but no local simulator or attached-device control path is available from these checks. The existing [device verification](./📓️device-verification.md) records no detected phone/tablet and no local Linux/Windows runtime.

The earlier [native entry audit](./NATIVE-ENTRY-AUDIT.md) records a Windows fleet host without a usable pinned compiler/worker and no Linux worker or mobile runner. These are prior snapshots, not proof of current fleet availability or physical execution. An attempted `repo remote-execution workers` remained in source admission and was stopped; no current fleet result was obtained. A worker registration alone would prove neither physical hardware nor an interactive input/display session. Obtain current worker status and host attestation, then run and record the native interactive test on actual Windows and Linux hosts and Safari/Android devices, including renderer, input event trust, map movement, screenshots and platform versions. No such physical proof is established here.
