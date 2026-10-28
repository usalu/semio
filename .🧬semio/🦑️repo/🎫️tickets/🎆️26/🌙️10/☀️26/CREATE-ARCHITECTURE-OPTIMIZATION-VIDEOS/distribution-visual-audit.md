# Distribution Visual Audit

Inspected actual frames decoded from compact MP4s under ticket/🗑️generated/delivery using registered FFmpeg and view_image. Canonical masters were not modified.

- Optimization_Extrema_speaker_de_1080p30.mp4:148.40s, H.264 High, yuv420p,1920×1080,30fps,AAC48kHz stereo. At63s, title, local/global labels, raised global label, both markers, formula, domain and complete two-line German caption remain readable. Same content and timing as previously audited master63s; no source crop or visible compression damage affecting reading.
- Optimization_Algorithms_speaker_de_1080p30.mp4:293.07s, H.264 High,yuv420p,1920×1080,30fps,AAC48kHz stereo. At163s, title, arithmetic-crossover heading, parental genes, child[0.30,0.70], mutation[+0.05,-0.05] and complete German caption remain readable. Same content and timing as previously audited master163s; no crop or visible compression damage affecting reading.

Evidence frames: ticket/🗑️generated/movie-audit/dist-extrema.png and dist-algorithms.png. Initial Algorithms probe occurred during encoding and correctly lacked a finalized moov atom; subsequent completed-file probe/extraction passed. Review is limited to the requested representative interiors; all18 master wrapper frames were audited separately in movie-audit.md.
