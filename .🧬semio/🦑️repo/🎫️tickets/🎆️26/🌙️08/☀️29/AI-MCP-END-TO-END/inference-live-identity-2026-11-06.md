# Inference Live Identity Admission

The actual Host router previously used BTreeMap::insert to detect a duplicate cancellation identity. A refused duplicate replaced the first active request's revision and generation, corrupting its final freshness authority. Admission now uses the vacant Entry branch, leaving occupied rows intact. The production infer path calls this single admission method.

A language-neutral schema and four-step fixture cover repeated identity rejection and independent admission. The TypeScript Map implementation and third-party SQLite unique-key insertion agree: one portable test passes with eleven assertions. A native law calls the actual router admission method and checks the real live commit map against the same fixture. That law is authored and awaits native qualification; no native pass or server-ready claim is made.
