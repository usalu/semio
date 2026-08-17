# UI Navbar Multi-Consumer Retention Audit

## Baseline

- HEAD: `5a1367dfcc90630c52dc2ec4de9526babe8d70f4`
- Navbar component SHA-256: `d8589ffe9779decca59616ae76c0f78d62279b737cb2a5f5f93f93629fccc7c8`, clean.
- Navbar story SHA-256: `bd3ff5b60d76cf43beac38a3b73b20b2d28ee09266bb47f0b49687608927e134`, clean.
- React index at audit completion: `64eb6dcf68e5c20a02409cedf789a96010f040d4144793b7de069f982795a10f`, accepted serialized UI changes only.

## Production Closure

At least two independent active production components render the React Navbar implementation:

1. Framework UI Canvas renders Navbar for its local canvas controls.
2. Protected OS renderer ShellHost renders Navbar for the application shell and supplies `NavbarItem` contracts.

The OS renderer package-index imports are glue, while Layout/Navbar stories and UI package tests do not increase the consumer count. Rust WGPU/TUI navbar states are separate language/rendering implementations rather than consumers of the React component.

## Decision

Retain Navbar at the framework UI owner. Its reverse closure reaches independent framework and product terminals, so the two-consumer threshold and lowest-common-owner requirement are met. Do not inline it into the protected renderer, delete it, or treat parallel Rust implementations as duplicate consumers. No source edit follows.
