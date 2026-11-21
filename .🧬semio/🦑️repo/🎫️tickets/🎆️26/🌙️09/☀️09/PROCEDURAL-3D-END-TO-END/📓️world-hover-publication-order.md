# Viewport hover publication order

The World3d renderer serializes instance, component and vortex hover updates through one pointer queue. Each offer captures the resolved interaction target. The latest local mark replaces any unsent mark. Leaving a viewport with local hover offers a clear on that same queue. A viewport that only displays an authored hover publishes no clear when the pointer leaves. Geometry, domain, utility and window changes discard obsolete queued targets. A geometry change retains local pointer ownership long enough to release it on exit.

The old renderer failed five fixture cases, including a queued face arriving after a clear and pending work publishing after unmount or a domain change. The final run passed 71 tests across the mounted World3d component, hover publication and refresh coalescing suites. The eight neutral ordering scenarios also passed against the independent RxJS oracle. Chromium rendered the four-instance fixture, published the qualified graph target and cleared the viewport while keeping geometry revision 1.

The linked application and native renderer still need their own end-to-end run. This Chromium witness drives the mounted World3d renderer through its scene boundary and dispatch output.
