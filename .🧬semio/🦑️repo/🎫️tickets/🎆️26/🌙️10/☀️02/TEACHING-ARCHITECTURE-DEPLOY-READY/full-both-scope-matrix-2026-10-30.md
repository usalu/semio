# Current Full BOTH Browser Scope

Read-only Source audit of course/e2e/config, e2e/sessions/JSON, runner and declared specs. No browser/test/probe launched. Source membership is intended coverage, not a pass.

| Project | Exact spec paths relative to course/tests | Main acceptance |
| --- | --- | --- |
| boot | 🚀site-boot/TS | Actual entry readiness, performed before every owned stack wave |
| desktop | 🪪first-visit/TS; 🥞layered-home/TS; 🎯quiz-runs/TS; ⛰challenge-levels/TS; 🏆live-leaderboard/TS; 🗣both-languages/TS | First visit, ordinary public locks/navigation; all10quiz perfect journeys and10×4challenge matrix, per-item feedback/badges/history; challenge timing; multiuser live rankings; locale gameplay/switching |
| phone | 📱phone/TS |375×812 touch/isMobile real navigation and gameplay |
| layout | 📐adaptive-layout/TS | Authored phone/tablet/desktop/wide1920 geometry, semantic result rows, no active horizontal overflow |
| pets | 🐕pet-walk/TS; 🐕pet-walk/🧭cast-and-choices/TS; 🐕pet-walk/🪜mischief-ladder-and-phone/TS | Real rendered cast/access, carry/drop/parachute/landing, companions/ladder/phone behavior |
| presence | 👥shared-presence/TS | Multi-context identity/presence/following/viewport behavior |
| shortage | 🔌connection-shortage/TS | Real backend outage/recovery, local answers and fresh-device restored answer comparisons |
| away | 📴proctor-away/TS | Arriving while actual backend is unavailable and eventual recovery |
| faults | 🛟proctor-faults/TS | Repeated actual stop/start during expert tasks/submission, fresh-device recall, deliberate contract-version mismatch/projection-lag cases |

These are9projects and16unique spec files. The table uses readable emoji names; exact variation-selector filenames are retained in the actual config.

| Topology | Owned stacks/waves and real linkage |
| --- | --- |
| release rehearsal | One actual production-mode proctor with allowed-origin equal to ephemeral static site. Qualified release executable, release rehearsal artifact retained by reader; artifact guards/budget pass before real static serving and browser boot. Shared wave then exclusive wave on this stack. Distinct browser/proctor origins enforce real production CORS/CSP linkage. |
| dev | Two isolated actual stacks, one shared and one exclusive, with separate ephemeral ports/data/control. Actual dev site/proctor and registered browser. Each stack warms boot before its wave; exclusive actions cannot stop persistent4262 services. |

Shard shared includes desktop/phone/layout/pets and may use admitted workers; exclusive is one worker with ordered dependencies presence→shortage→away→faults. --serial sequences topologies, not deletion of a shard or forcing every browser to one worker. All service mutation controls belong to those throwaway stacks, not managed6061/8791. Fault-version cases intentionally intercept instance metadata; they do not replace the main gameplay backend with a mock.

Persistence boundaries:10×4 pool/result history checks include same-context reload; first-visit/shortage/faults supply fresh empty-context identified recovery and actual backend restart. Pending42read maintenance proof is separate. The full10×4 matrix is English; both-language journeys cover all screen translations and representative task kinds plus both software quizzes, not an asserted10×4×2 gameplay cross-product. Selected all-content journeys use explicit authorPreview; public lock/access laws remain distinct.

Config defaults: retries0, forbidOnlytrue, pass/failure strictly reported, test600000ms/expect20000ms, no webServer. Scoped search found no test.skip/test.only/describe.skip/test.fixme, defaultgrep or testIgnore in declared browser Source/config. Runner accepts explicit forwarded Playwright filters after --, so full acceptance must retain empty forwarded args: no grep/project/file/only-changed or --proctor override. Default plan includes both topologies; exact required command remains repo run @teaching/architecture-course:test-e2e --configuration=release --parallel=1 -- --topology both --serial. Normal runner records all wave statuses and only concludes success when all are0; cancellation or partial shared progress is not completion.

Current full React73624 executing is independent unit evidence;43component files and this16browser-spec matrix must not be confused. Historical48540 partial59pass12fail/canceled130 remains the baseline, not a current full BOTH result. Current Records ticket diagnostic is supplemental only.
