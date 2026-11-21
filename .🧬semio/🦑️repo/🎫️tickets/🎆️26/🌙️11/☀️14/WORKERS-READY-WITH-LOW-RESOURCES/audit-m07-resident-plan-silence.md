# M07 Resident Plan Silence

The two latest mirrored M07 workspace attempts ended after approximately three 120-second client intervals with `HostUnavailableErrorV1("silent")`. Those records predate the diagnostic change. They do not identify whether the connection received a welcome, whether the host accepted the exact plan request, or whether accepted work stopped reporting progress.

## Request evidence

- A valid service welcome establishes the connection phase. The service sends an `accepted` frame with the request ID after registering that exact plan and before starting workspace identification. Matching valid progress also proves acceptance. Silence now reports the last proven phase and, when available, the last matching progress phase.
- Duplicate welcomes and unrelated or malformed acceptance and progress do not defer the silence timer. A refusal for another request does not end the exchange. The normal connection-wide refusal and solicitation of a matching slice report still work. The five extra reattachment attempts still require matching valid progress.
- Production retains the 120,000 ms interval, the default three attempts, and the existing service keepalive cadence. The existing production progress phase `slice` is now represented in the frame schema. Native English and German messages report the stage in the requested language.

## Verification

- The existing implementation failed the private regression because the errors omitted stage and limit, unrelated progress extended retry, and a wrong-request refusal ended the exchange. The final regression used eight neutral scenarios and an independent SQLite stage oracle, then held an actual private resident planner through three matching service keepalives and a normal plan reply.
- The focused socket and protocol suites passed 24 tests and 678 assertions. Protocol verification included 2,526 mutated frames compared with Python jsonschema, and an Ajv check.
- The initial red harness accidentally opened six fixture-only runtime directories in the worktree root because Bun marks a directly specified test file as the main module. The child now requires its explicit fixture argument. The six identified private remnants were removed after checking their exact paths and ownership evidence. The final tests ran in ticket scratch. No live service or remote worker was changed.
