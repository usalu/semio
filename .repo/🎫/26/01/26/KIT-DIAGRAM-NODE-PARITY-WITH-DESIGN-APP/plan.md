# Plan - Kit Diagram Node Parity

Scale Kit diagram nodes to 2× size while keeping the circular Design parity, and ensure relationship lines intersect the scaled circle outline.

## Steps

1. Update `NODE_WIDTH` and `NODE_HEIGHT` in `js/semio/sketchpad/Kit.tsx` to `ICON_WIDTH * 2`.
2. Ensure the rendered avatar circle matches the node box size (pass `className="size-full"` to `TableAvatar`).
3. Keep `getNodeIntersection` circular and ensure it uses the scaled radius (`NODE_WIDTH / 2`).
4. Tune the force layout minimums (link distance + collide radius) so larger nodes don’t overlap.
5. Update dev docs (`README.md`, `AGENTS.md`) to reflect the Kit diagram node sizing + edge intersection mechanism.
6. Verify visually in the Kit diagram window.
