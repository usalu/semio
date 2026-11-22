# Crane export and repeated geometry

The open-truss carriage candidate originally occupied 8,849,064 bytes. A binary accessor inventory identified 1,067 repeats, comprising 435,540 bytes of duplicated payload. Exact short-accessor sharing above a 128 KiB buffer size reduced the same candidate to 8,244,640 bytes. The number of accessors fell from 1,607 to 552. Twelve repetitions containing 3,864 payload bytes remained. The revised candidate with both rope falls occupies 8,246,056 bytes. The publication limit is 8,388,608 bytes.

The writer bounds its short-accessor lookup to 4,096 entries and compares type, component count, bounds, target and actual bytes before sharing. The neutral fixture covers equal and distinct vertices and index accessors in a large document. An independent glTF reader reopens the resulting buffer. The four GLB test suites passed all 51 tests.
