# Architecture course CDN package

The course’s release validator accepted a fresh Vite build and staged 116 files. The compressed package at `/Users/ueli/Documents/Codex/2026-11-19/goal-get-architecture-course-website-and/outputs/course-cdn.tar.gz` contains the site at archive root, including `index.html`, the matching Pages fallback `404.html`, `.nojekyll`, `CNAME`, `_headers` and six localized assignment PDFs. Each archived file was byte-compared with the stage. The SHA-256 of the archive is `96acb2f899dae1bb42a58de2038d8384f6dffe54f8260fcc20dadc6f211234cb`.

The release uses `https://architektur-und-technologie.de` and the proctor at `https://semio.iek.uni-hannover.de`. The graph-based release target stalled during graph slicing and was cancelled. The documented Bun/Vite fallback built the frontend from the current sources, fed the six existing PDFs from the deployment checkout through the regular Vite assignment channel, and called the regular `siteArtifactProblems` and `stageCourseSiteArtifact` functions. All six PDFs have three to five pages. The deployment checkout was used read-only.

The course unit suite passed 875 tests across 20 files. All 45 local CSS asset references resolved in the staged directory. A Chromium test against the static artifact fetched and checked all 26 learning-clip posters and opened the PDF iframe for each of the three assignments through its hash route.

A separate preview build, served on `http://127.0.0.1:19880/`, points at the local proctor on `http://127.0.0.1:19882`. Its release validator passed, the root returned 200, and a missing path returned 404 with the course shell. The production archive hash stayed the same.

Offline Chromium smoke passed all 15 checks on desktop and all 15 on a 390px phone viewport. The browser covered the five released energy quizzes, the quiz catalog, leaderboard and badges views, introduction, language, theme, pets, settings, and PDF delivery. Public proctor requests were intercepted to exercise the offline UI. The separate local preview stays available.
