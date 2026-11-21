# Native observation presence child fixture

The original owned child exited before reporting its outcome. Its private workspace initializer statically imported the build-test invocation receiver, which registers a Bun `afterAll` hook. The literal compiler-captured child runs as a Bun script. It now uses the existing runner-independent guard to validate its inherited private workspace, while retaining the same process invocation Original and capture path.

The first runnable ancestor-absent case then reported two computations, a warm first reuse, and no predicate failure. The fixture removed the inventory namespace before the predicate could observe it. The inventory reader treats that initial absence as a cold miss. The fixture now moves the physical namespace after its ancestor identity is sampled so revalidation receives an actual ENOENT. The adjacent replacement case still recreates the namespace with a different identity. Existing outcome assertions and the independent SQLite oracle remain in place. The parent also checks that the absence fault actually ran.

The exact ancestor-absent source case passed under managed native custody, and the fixture changes passed the diff whitespace check. Production behavior and the live graph reader were untouched.
