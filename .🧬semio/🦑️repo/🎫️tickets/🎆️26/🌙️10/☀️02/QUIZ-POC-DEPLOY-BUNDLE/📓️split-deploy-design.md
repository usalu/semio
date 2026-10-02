# Split Deployment and Offline Backend — design

Request (2026-10-02): the frontend and the backend of the architecture quiz are deployed separately — the frontend as a
static page over a CDN at `quizze.architektur-und-technologie.de`, the backend on the Linux machine
`semio.iek.uni-hannover.de` (130.75.188.198) with Docker — the frontend works with an offline backend (changes are saved
locally), everything end to end, and both deliverables are handed over as folders.

## What was found

- The deploy tooling already splits site and proctor (`publish` stages the CDN folder, the image carries the stack), but
  names `quizzes.…` and `proctor.quizzes.…`. The one-origin `poc` bundle of the first round contradicts the split.
- `semio.iek.uni-hannover.de` resolves to 130.75.188.198; ports 80 and 443 time out from outside the university
  (probed 2026-10-02). Either nothing listens yet or the university firewall drops inbound traffic. Consequences:
  Let's Encrypt can only issue when 80/443 are reachable from the internet, so the stack must also take a certificate
  the operator supplies; and learners outside the reach of the host meet exactly the "offline backend" case.
- The client needed the proctor for everything but answers: the catalog, registration, starting a run (the sheet),
  submitting (the score, the badges). Only `record-answer` commands waited in an outbox. With the proctor away a first
  visit showed "Loading…" forever.
- The quiz core has bit-exact TypeScript twins of every decider and view (`decideLearner`, `evolveLearner`,
  `learnerView`, `runView`, `catalogView`, `leaderboard`, `sheetOf`, `scoreRun`, `earnedBadges`; seed = FNV-1a of the
  run id), so a device can decide exactly as the proctor does.

## Decisions

### 1. Hosts

`🚀️deploy/🔣️.json`: `site.host = quizze.architektur-und-technologie.de`, `proctor.host = semio.iek.uni-hannover.de`.
Every file that repeats them follows; the drift check covers the zones of both hosts.

### 2. The deputy: the device decides while the proctor is away

A site hands the quiz client its material (the catalog and the quizzes with their solutions). With it the client has a
**deputy**: the proctor's own deciders run on the device.

- A command goes **straight to the proctor** while the proctor answers and nothing the deputy decided is still waiting
  (so nothing overtakes). That path is unchanged: the proctor's verdict is the answer.
- Otherwise the **deputy decides**: the learner's state is derived from the views the client holds
  (`learnerStateOf(learner view, run views)`), the core decides, the events are folded into the views at once, and the
  command waits in the outbox. A direct attempt that fails for a connection shortage falls to the deputy.
- The **outbox holds every command** (not only answers), one record per command, in order; answers still coalesce per
  run and task. Delivery is sequential under the command's own id, so a retry is applied once.
- What the deputy decided is **provisional**: once the outbox holds nothing of its kind any more, the learner and run
  views are read from the proctor again and replace the local ones. While something waits, the proctor's views are not
  adopted (they are behind).
- A pseudonym or name registered by the deputy may turn out to be claimed. By the recall rule (whoever enters a claimed
  handle continues as its holder) the device then **continues as the holder**: the waiting commands and the cached views
  are re-addressed to the holder before the next command is sent, and the learner is told.
- A run the proctor refuses to start (an open run of the same quiz elsewhere, a cap) is voided on the device with the
  usual notice.
- The catalog shows at once from the material; the proctor's catalog replaces it when it answers.
- While the proctor is away the leaderboard shows this device's own standing, marked as such; a standing of the proctor
  that is already held is never replaced by it.
- A site without material behaves exactly as before.

Solutions travel in the site's scripts with this. That is inherent to scoring without a server; the proctor still
scores every run itself, so the leaderboard never trusts a device.

### 3. The proctor stack on the university host

- `compose.yaml`/`Caddyfile` name the new hosts. Caddy loads certificates from `./certificates` (PEM bundles:
  certificate chain and key in one file); when one matches the host it is served and no ACME order is made, otherwise
  Caddy obtains one itself (needs 80/443 from the internet).
- `docker-stack-bundle` stages what the host needs without a registry: `compose.yaml`, `Caddyfile`, `certificates/`,
  `.env`, `proctor-image.tar`, `README.txt`. The one-origin `poc`/`poc-check` verbs and `🚀️deploy/🛫️poc` go.

### 4. Proof

- Unit: deputy round trips against the lifecycle vectors; outbox with every command kind; session journeys without a
  proctor (first visit, run, submit, badges, reload, sync, handle collision).
- End to end: a spec that takes the proctor away before the first visit, plays a quiz to its result, reloads, brings the
  proctor back and finds the result on a fresh device — in both topologies of the gate.
- Deliverables: `publish` output served and driven in a browser; the bundle run with `docker compose` on this machine.
