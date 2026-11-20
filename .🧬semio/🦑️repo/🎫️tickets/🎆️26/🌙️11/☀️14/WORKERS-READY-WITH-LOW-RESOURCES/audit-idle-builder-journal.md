# Attached readers and cancelled graph waves

Two isolated tests expected every intermediate inference wave to update `current.json`. Current generation building records intermediate work in the checksummed build journal and publishes the canonical pointer after sealing. The old abort fixture waited for two pointer increments until the graph had already finished. The old attachment fixture saw one valid seal and required two.

The attachment test now verifies one observed seal, zero inference by the attached reader and graph equality against the Python oracle. The cancellation fixture observes the real second durable journal header after its file write and aborts then. It verifies that the previous sealed pointer stays current, the next read resumes the two recorded waves, only the remaining owners are inferred, and the final graph equals the Python oracle. The schema’s English and German descriptions now identify which progress the journal saves and when the canonical generation is published.

All seven tests passed with 43 assertions.
