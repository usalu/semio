# Split Deployment and Offline Backend — summary

Request (2026-10-02): frontend and backend of the architecture quiz deployed separately — the frontend as a static page
over a CDN at `quizze.architektur-und-technologie.de`, the backend with Docker on `semio.iek.uni-hannover.de`
(130.75.188.198) — the frontend works with an offline backend (changes saved locally), end to end, and both deliverables
handed over. Design: `📓️split-deploy-design.md`. Deploy tooling in detail: `📓️deploy-retarget-report.md`.

## The deliverables

Both are staged by verbs and copied to one stable, git-ignored place (a site build empties the package's `dist`):

| Deliverable | Where | Staged by |
|---|---|---|
| the static frontend (63 files, 2.7 MB) | `.🧬semio/🎓️teaching/architecture-quiz-poc/site` (= `…/📦️packages/🟦️typescript/dist/pages/quizzes`) | `bun nx run @teaching/architecture-quiz:publish` |
| the backend bundle (32 MB) | `.🧬semio/🎓️teaching/architecture-quiz-poc/proctor` (= `…/dist/proctor`) | `docker-image-build`, then `docker-stack-bundle` |

- **Frontend:** upload the *content* of `site` to the document root of `quizze.architektur-und-technologie.de`. It is
  baked for `https://semio.iek.uni-hannover.de` and talks to nothing else (Content-Security-Policy). Pages are addressed
  by hash, so no rewrite rule is needed. Some files have emoji names (`🖼️assets/…` holds the cursors): the upload must
  keep UTF-8 file names.
- **Backend:** copy `proctor` to the machine and follow its `README.txt`: `docker load --input proctor-image.tar`, a
  certificate (Let's Encrypt by itself once 80/443 are open from the internet, else a PEM bundle in `certificates/`),
  `docker compose up --detach --wait`.

## What was built

1. **Hosts** (`🚀️deploy/🔣️.json` and every file that repeats them), drift check over the zones of both hosts, a
   certificate of the operator's own (`tls { load /certificates }`), `docker-stack-bundle`; the one-origin `poc` verbs
   and `🚀️deploy/🛫️poc` are gone. (Agent work, see its report.)
2. **The deputy** (`🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🔨️modules/🫡️deputy`): with the site's material
   (`🎓️teaching/🏛️architecture/❓️quiz/📚️catalog/🟦️.ts`: catalog + quizzes) the core's own deciders run on the device.
   The session (`🧭️session`) sends a command straight to the proctor unless the proctor is known away or the device is
   ahead; else the deputy decides, the events fold into the held views and the command waits in the outbox, which now
   holds every kind of command in order (`📮️outbox`). A learner waits 3 s at most for a silent proctor. Delivery
   re-decides everything at the proctor; a claimed handle makes the device continue as its holder; a run the proctor
   will not start is voided on the device. Leaderboard without the proctor: the device's own standing, marked.
3. **Texts** in English and German: connection ("Quiz server not reachable – everything is saved on this device"),
   leaderboard ("Only this device – …"), the recall notice, what the browser stores.
4. **Docs:** quiz product README "While the proctor is away"; site README "While the proctor is away", "Proof of
   concept by hand", Deploy.

## Evidence (all run 2026-10-02 on this machine)

| What | Result |
|---|---|
| `@semio-tech/quiz-react` unit tests (`bun ./📜️script.ts test`) | 20 files, 640 tests pass — among them 8 deputy journeys in `🚶️learner-journey` (first visit without proctor → reload → sync; direct while the proctor answers; proctor away mid-run; silent proctor and patience; handle already known → holder; run the proctor will not start → voided; two tabs; app-level texts), 11 in `🫡️deputy-decisions` (the lifecycle vectors of the Python reference decided from views alone equal the core's decisions from the stream), 4 new in `📬️outbox-delivery` |
| `tsc` of `@semio-tech/quiz-react` and of `@teaching/architecture-quiz` | exit 0 both |
| `@teaching/architecture-quiz` node tests (`test`) | 5 files, 133 tests pass (catalog incl. the material module, deploy, local stack, host document, pet cast) |
| taxonomy report, scopes `🎓️teaching` and `🧰️framework/🛍️products/❓️quiz` | clean both |
| end-to-end gate, projects `presence`, `shortage`, `away` (`test-e2e --project … --no-deps --workers 1`) | 5/5 pass in `dev` and in `rehearsal` — `📴️proctor-away`: the proctor stopped before the first visit, a quiz played to 100 % with feedback and badges, kept over a reload, the proctor started, a fresh device finds the same learner and result; and the claimed-pseudonym case |
| `docker-image-build`, `docker-image-check`, `docker-stack-check` (with the supplied-certificate proof), `publish`, `docker-stack-bundle` | exit 0 each |
| `prove_split_stack.ts`: the staged bundle run with `docker compose` (host `localhost`, Caddy's internal CA) + a static release build of the site on another origin, in Chromium | backend up: registered and played, "All answers saved" → backend stopped: "Quiz server not reachable – everything is saved on this device", another quiz played to 100 %, reload keeps it, 4 commands wait → backend started: "All answers saved", nothing waits, a fresh device finds the learner with both quizzes at 100 %; no page error, no policy violation |
| `drive_published_site.ts`: the published folder itself (baked for the real `https://semio.iek.uni-hannover.de`, which does not answer from here) served as static files, in Chromium | overview 4.7 s after arriving (3 s of them the patience for the silent host), cooling played to 100 % with its badge, reload keeps it, leaderboard shows the device's own standing, header: "Quiz server not reachable – saved on this device, still to send: 5"; no page error, no policy violation |

## Not proven, and what is not mine

- **Outward steps** cannot be rehearsed here: the upload to the CDN, 80/443 of the university host from the internet
  (they timed out from outside on 2026-10-02), Let's Encrypt, a certificate of a real authority. Until the university
  firewall opens 443, a learner outside the campus network never reaches the backend — the site then stays in the
  "saved on this device" state for them, by design.
- **The whole end-to-end gate does not pass at the moment, for reasons outside this ticket.** In a full run three specs
  of other sessions' work fail and stop the gate before the projects above: `🗣️both-languages` (the results of heating
  show figures headed by values such as "15 kWh/(m²·a)", which are the same in both languages — it also fails run
  alone), `🏆️live-leaderboard` (expects the highest total to be 100 while the perfect player of `🎯️quiz-runs` reaches
  400 beside it — passes run alone) and `📱️phone` in `rehearsal` (the leaderboard card grows beyond half a screen once
  other specs filled the leaderboard — passes run alone). `deploy-check` runs that gate as its last step and therefore
  stops there too.
- The solutions travel in the site's script with the material. The proctor scores every run itself.
- What a device holds stays there until the proctor has it; clearing the browser's data before that loses it.
- A learner whose proctor lost its data (a database reset during the proof of concept) is asked to identify again, as
  before; the device does not re-send what the proctor had already accepted.
