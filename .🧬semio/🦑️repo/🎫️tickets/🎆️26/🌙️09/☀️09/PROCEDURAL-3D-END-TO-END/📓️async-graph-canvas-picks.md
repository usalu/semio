# Graph canvas pointer picks

The Flow graph now waits for the native pick at the current pointer position before using it. Its graph hover publication waits for the corresponding native hover write. A newer pointer query or incoming scene revision invalidates an older reply. Pointer leave, explicit gesture cancellation and unmount abort pending hover work. Pending click picks can settle while the mouse moves. Cancelled gestures clear the start point so the next hover can run. The synchronous graph and paint routes keep their immediate target lookup.

The Flow task observer now checks that a native completion still owns its session and feature before delivering it. This also handles a worker that returns a result after cancellation. The registered hook and actual Flow adapter tests passed 19 checks across three files. The independent RxJS switchMap oracle passed nine neutral cases. TypeScript parsed all ten changed source and test files. The final Chromium regression retained graph-qualified world hover, four-region clear behavior and the original geometry revision. The probe now waits for the canvas backing dimensions before sampling.

The viewport browser witness covers the mounted World3d renderer. The Flow adapter witness uses controlled native task replies. The whole mounted application and the registered native port-row geometry still require their own verification.
