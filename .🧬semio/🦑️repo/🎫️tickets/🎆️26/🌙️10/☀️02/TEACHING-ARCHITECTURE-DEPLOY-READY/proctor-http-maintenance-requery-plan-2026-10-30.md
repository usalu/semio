# Read-Only Persistence Requery After Managed Restoration

Execution is held until frontend confirms actual managed owner 4262 and both HTTP listeners ready against the standard proctor-dev datastore. No replacement server or data operation is authorized by this test.

The retained input proctor-http-all-quizzes-restarted.mjs now requires an explicit new output basename and current owner description, preventing a stale default 68016 label or accidental overwrite of the earlier receipt. It uses only GET /instance and 42 POST /queries authority reads: 40 exact run IDs, one learner and one leaderboard.

Current schema binding is course/schema/TS RunView, RunSummary and LearnerView. Every run must match its returned run ID, learner, quiz and sheet/result quiz plus challenge, submitted status, score, points and answer count. The learner must match its ID and the entire sorted 40-run identity/quiz/challenge/status/score/points projection. The prior exact best points and badges checks remain. The test explicitly requires 42 queries, 112 answers and 13 badges.

This proves persisted identities and summary/count equality across the recorded expiry and managed restoration; it does not compare the historical full answer bodies, which were not retained in the baseline. It does not mutate course state or read the SQLite file. The actual HTTP proof remains pending readiness.

Registered Node --check exited 0. Input SHA256: 6dc8d154d4cfa9fcab367aaa1dd730b66611d35cc95eb0a6db0623024d22f145

Receipt creation now uses exclusive wx creation, so an already-existing basename is refused rather than overwritten. No receipt file was created during preparation. Final input SHA256: 9f34b9d76ec2fa99c47df4dcd3aafbbc42b0a217210307d48ebda0e8cedf5dc5.

## Actual Host-Reboot Restoration Result

Registered Node execution exited 0 after frontend confirmed actual managed87807 proctor8791 and Vite6061 readiness. The exclusive new receipt is generated/proctor-http-host-reboot-restarted-2026-10-30.json (SHA256 667164f7215a4747dc1b445871fa8b0c88afef7b67c595ccf9d6f872de77f3bd). The same learner and exact 40-run roster survived the original backend expiry, controlled stack maintenance and actual host reboot. All returned run/learner/quiz/challenge/status/score/points identities matched; 42 authority queries verified 112 answer entries, 4000 best points, all 10 quizzes at all four levels, 13 badges and own leaderboard. Every query response retained exact CORS origin http://127.0.0.1:6061; GET /instance was teaching-proctor. Mutations and direct database reads were both zero. No historical full-answer-body equality is claimed. The managed service remains frontend-owned and live; no service was stopped or restarted by this check.
