# Mounted FEM live maintenance

The browser cleanup loop previously scheduled mounted maintenance on every turn that retained the app lease. An idle mounted FEM 3D app could therefore return zero-progress retirement work indefinitely. The FEM 2D editor used the same phase condition.

The 2D editor, 3D editor, and 3D viewer now advertise maintenance when a detached consumer, unreferenced request, or eligible retiring child exists. FEM 3D also checks for a completed replacement result and detached recovery owners. Live maintenance completion leaves the active app and its result custody intact. The app-close witness still checks that every app-scoped owner was released.

The 3D walk selects the next matching retirement or recovery shell across the fixed registry. Empty lookup positions no longer generate zero-progress maintenance turns. Closing a numerical cursor rung or removing an empty solver owner reports that control transition. The 2D and 3D snapshot close paths report an input wait while the original snapshot return remains outstanding.

The neutral lifecycle fixtures distinguish a live idle app, detached retirement, idle after retirement, and strict app close. NumPy reproduced each fixture's pending and terminal pair from the app-lease and detached-owner inputs. Native execution of the mounted session regression is pending in the Windows Cargo build.
