# Real HTTP Learner Workflow — 2026-10-29

The canonical persistent Cargo runtime41024 remained live throughout this bounded check. No server restart, heavy build, Native job, direct datafile access, Source change, peer stop or cleanup occurred.

## Supported Contract And Authored Inputs

The ticket script uses the existing real HTTP gateway contract from proctor end-to-end harness: POST /commands with course.<command>, wire version10, learner actor target, anonymous identify principal and user principal for learner commands; POST /queries with anonymous authority snapshot envelopes. Every request includes the allowed localhost6061 origin and asserts its CORS response. IDs are fresh random32hex identifiers.

Input proctor-http-journey.json specifies all four existing challenge levels and100/200/300/400 maximum points. AJV validates its closed shape; Lodash independently generates expected challenge points and computes sorting order/classification assignments from the actual authored physics quiz. Test-only solution access stays in the ticket client; no answers/solutions were added to the API or logged in summaries. The public open-sheet checks assert items contain neither numeric value nor category solution fields.

## Actual Mutation And Assertions

At13:25:49UTC, registered Node execution exited0 in0.33s. It created one isolated anonymous test learner in the standard development data through identify-learner. It then completed four physics runs sequentially (easy, medium, hard, expert), each with all three presented tasks: classification and two sorting tasks. All12 real answers were accepted. The expert run opened all three timed tasks through open-task before answering. Each answer command was resent with its identical command/idempotency ID: all12 retries returned no events.

All four submit-run commands were accepted, produced run-submitted, and returned score1 at the independently expected100/200/300/400 points. The query views were submitted and held all three answers. Four run views and the learner view were compared with fresh authority re-queries (five stable comparison pairs, ten reads participating in those comparisons). The learner has four runs, best physics challenge expert, and total400 rather than the sum of superseded bests. Its all-time leaderboard own row matches400, and the public board contains no raw learner ID.

There were36 accepted command HTTP requests: identify1, starts4, record requests24 (12unique +12same-ID retries), timed opens3, submits4. This is24distinct command IDs; the response event checks specifically establish duplicate answer commands emitted no new events. Badge emissions were allowed but no specific badge count was asserted.

At13:26:47UTC, a second independent registered Node process made five fresh HTTP authority reads of all four run IDs and the learner. It again asserted submitted state, score1, exact points, four learner runs and400best total, with CORS for the second allowed origin127.0.0.1:6061. It exited0 in0.085s. It did not reuse client state or read the server database.

## Evidence And Limits

Inputs retained: proctor-http-journey.mjs and proctor-http-journey.json. Generated evidence: proctor-http-journey.log, proctor-http-journey-result.json and proctor-http-journey-requery.json. The first result's authorityRequeries:10 field counts reads participating in the five comparison pairs; it is not ten repeated-query pairs. The separate independent-client receipt reports its five actual reads explicitly.

The owned test learner and four completed runs remain persisted in the development store as deliberate test data; no operator erase command was issued. Only opaque IDs and score summaries are logged, never solution contents. Persistence scope is repeated authority queries across client processes against the same live server. No claim of crash/restart durability, browser interaction, public deployment, complete course coverage or qualified Nx build success follows. These checks complement pending normal browser gates. Parent independently confirmed basic8791 readiness; session41024 remains owned and live for that verification.

The independent read-only check is also retained as proctor-http-requery.mjs and was executed again from that exact file at 2026-10-29T13:27:46.148Z (exit0,0.107s), writing proctor-http-requery.log and the current five-read receipt. It does not repeat any learner mutations.
