# Native Renderer Return Laws

The renderer's native I/O and independent decoder tests now use the same funded return origin as the production interactive job driver. The fixture admits and closes each origin on its original caller. The page test receives the descriptor, borrows its output, acknowledges it, and verifies the same page allocation enters World and returns to the source. Page transfer and retirement measurements run inside the caller turn, after the source refresh.

The decoder tests distinguish an admitted turn from an admission refusal. One unit of fuel admits a callback and observes exactly 16 KiB. A zero-item loan and external cancellation each produce `Preparing` without entering the callback or changing its retained receipt. Both paths preserve the exact probe and allow the origin to close on its source. The separate mounted worker test still verifies the public cancelled result from session close.

Both Rust test modules parsed with the repository-pinned `rustfmt`. The scoped `git diff --check` passed. Native Cargo execution awaits shared machine capacity.
