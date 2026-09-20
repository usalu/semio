# Event Feed Time Formatting

The remaining EventFeed difference is confirmed by current source. React `formatFeedTimestamp` constructs `Date(timestampMs)` and calls `toLocaleTimeString([], { hour: "2-digit", minute: "2-digit", second: "2-digit" })`. It therefore uses the host locale and the host time zone at that instant, including DST and the locale's hour cycle. WGPU Scenes calls `event_feed_time_of_day_utc`, taking milliseconds modulo86400000 and formatting24-hour UTC. Both suppress a zero timestamp at the rendering site.

A fixed numeric offset or manual24-hour formatting would still miss React's contract. The execution packet should expose an owned host-time formatting interface, with browser Date/Intl behind the browser implementation and native system formatting behind platform implementations. Do not export third-party types, add a UTC fallback presented as parity, or globally mutate the process time zone during parallel tests.

A neutral fixture should cover nonzero timestamps in winter and summer, en-US and de-DE hour cycles, and at least UTC/Europe-Berlin/America-New_York. Obtain expectations from actual installed Intl/React rendering with an explicitly controlled environment. Verify the production browser worker receives the same host context as React, and that invalid/out-of-range timestamps obey the actual reference behavior. This audit made no production change and claims no native time-formatting validation.

Source locations: EventFeedHost React lines30–35; Scenes WGPU `event_feed_time_of_day_utc` around3874 and render call around3978. A scoped framework Rust search found no existing localtime/chrono::Local/timezone-offset utility to reuse; this is not a repository-wide absence proof.
