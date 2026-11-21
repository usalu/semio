# World3d hover in Chromium

The Playwright probe mounted the production React World3dHost, R3F and WebGL2 in Chromium. It used the registered neutral four-instance fixture and injected the authored selection at the scene boundary. The scene geometry used four separated visible objects, with two graph channels and repeated instances of a shared mesh.

Three authored regions changed by 471, 625 and 625 pixels respectively within their 25-by-25 sample regions. The unrelated region changed zero pixels. A real ray to the unrelated instance published interactionHover to the graph domain and pointer channel with handle unrelated@body. The three authored regions returned to their initial paint and the local region changed 445 pixels. Moving out of the viewport published an empty target list and cleared all four marks. Geometry revision stayed at 1.

The probe runs via repo tools exec --light bun with world3d-browser-probe.ts in this ticket. Its source mount and HTML are retained here, and the probe copies them into the ticket's generated directory. The probe covers the browser renderer and its outbound graph event. The complete Generation3d app, graph-to-world bridge, and native host still need their linked end-to-end runs.
