# Intro Skip and identity input under extreme load

Verdict: not a product defect. The lost or unresponsive Skip click and identity input are a harness timing effect of an overloaded host. I changed no product code and added no specs.

## Method

- I built the release site with `chaos-build.mjs`, with the proctor at 8799 and the site at 6068, ports 8799 and 6068 only.
- I ran it with `chaos-stack.mjs`: `STATIC=1`, `PROCTOR_PORT=8799`, `SITE_PORT=6068`, a copy of the proctor binary, and work folder `🗑️generated/intro`.
- Each trial is a fresh Chromium context with CDP `Emulation.setCPUThrottlingRate` at 20 or 50 and `Network.emulateNetworkConditions` (150 ms latency, 1.6 Mbit/s down, 750 kbit/s up). No retries anywhere.
- Scripts kept in this folder:
  - `intro-load.mjs` runs the whole first visit. It clicks Skip once, opens the sign-in card, checks the "name" radio, fills a name and waits 3 s. Mode `stable` uses Playwright's actionability click; mode `raw` uses `mouse.click` at the button's first-visible position.
  - `intro-probe.mjs` records the Skip button's rect on every frame, plus the pointerdown, pointerup and click events and their targets.
  - `identity-probe.mjs` clicks the radio at its position and types at once, with no waits. It also watches whether the dialog is ever replaced.

## Results

| Probe | CPU slowdown | Trials | Outcome |
| --- | --- | --- | --- |
| Skip, stable click | 20 | 4 | 4 of 4 dismissed; the click took 4.4 to 14.4 s to be delivered |
| Skip, stable click | 50 | 2 | 2 of 2 dismissed; the click took 19 s and 35.6 s |
| Skip, raw click | 50 | 2 | 2 of 2 dismissed; the click took 21.9 s and 46.1 s |
| Skip rect probe | 20 and 50 | 4 | Rect never moved after first show (x 808.45, y 0, 45x22); pointerdown and pointerup hit `introduction-close`; the click event fired; intro gone |
| Identity step, radio check and fill | 20 and 50 | 6 (`intro-load`, 4 stable and 2 raw) | Radio stayed checked, `Load Tester` stayed in the field after 3 s, no remount |

Dialog identity probe: the identity dialog was never replaced (0 remounts) in the 3 trials that reported one.

The gap between pointerdown and pointerup was 1.7 to 1.8 s at 20x and 3 to 240 ms at 50x. Both events landed on the same element.

I killed the 50x stable runs at trial 2 because every trial passed.

## Candidate causes, checked in code and by measurement

- **Handlers not live before the controls render.** There is no static shell. The intro is client-rendered inside `CourseApp` and appears only once the lazy `UIIntroduction` chunk, the course and `HomeScreen` are loaded (`📚️app` returns `waiting` until then). A visible Skip button is therefore live.
- **Layout shift.** The probe saw no rect change from first frame to dismissal at 20x and 50x. The box starts at its measured position.
- **Overlay interception.** The veil under the box handles `onPointerDown={skip}` when it blocks pointer, so even a missed click dismisses. No pets, toast or tint layer sits over the box during the intro. The dialog layer (`z-50`) is only mounted after the intro is gone.
- **State reset by a late course or language load.** `course-loaded` replaces `state.course` and recomputes the intro definition, but `stepIndex` lives in `CourseIntroduction` and `IdentityScreen` has its own state; neither remounts (0 remounts observed). `introduced` is only ever set to true by the reducer.
- **Disabled-while-loading control.** Only the identity submit is disabled while working, with a visible status and cancel.
- **Controlled input overwritten.** `handle` is local React state set by `onChange`; nothing else writes it.

## Why the harness lost input

- At 50x CPU on a host with load average 80 to 290, the page needs 20 to 46 s to take one click end to end (Playwright's stability and hit checks wait on `requestAnimationFrame`, which runs at a crawl). The intro helper's `click({ timeout: 10_000 })` and `expect` timeouts of 10 to 20 s expire before the page answers.
- The input is not lost in the page: it is delivered late. A retry or longer timeout is the correct fix, and the harness retries now.

## Real-user observations, not changed

- Pointerdown on the intro box costs about 90 ms of main thread per 1x (surface-active re-render). That is 1.8 s at 20x. It does not lose the click, and the cost only shows on a very slow device.
- After a pointer click on the "pseudonym" or "name" radio, keystrokes typed at once go to the radio, not the new field, because focus stays on the radio (`identity-probe.mjs`: `text` empty, focus `INPUT:radio`). This is standard radio semantics and keeps arrow-key navigation among the kinds; moving focus would break it. Playwright's `fill` focuses the field, so the harness is not affected.

No unit test or e2e spec was added, because no defect was found.
