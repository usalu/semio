# Native Image Event Log Review

The lease-image census observed system `.fseventsd` allocation separately from ordinary lease databases. Root searched the current process storage modules for event-log or metadata-index suppression and found no such production policy. Existing image operations delegate creation/activation/inspection and use bounded owned hdiutil commands. No system event files or operating-system service settings were changed.

Apple's archived [File System Events guide](https://developer.apple.com/library/archive/documentation/Darwin/Conceptual/FSEvents_ProgGuide/UsingtheFSEventsFramework/UsingtheFSEventsFramework.html) describes persistent volume events. Its cleanup section documents a root-only historical-event purge API and cautions that consumers may share that history; it does not establish a documented per-image no-log provisioning recipe in the inspected page. The archive is dated2012-12-13 and is not proof of current host implementation behavior.

This read-only research does not justify deleting `.fseventsd`, stopping a system daemon or relying on an undocumented marker. The observed pressure will be addressed through framework-owned ordinary lease lifecycle and finite accounting. System event-log allocation remains an observed capacity contributor, with no actor or event-rate attribution.
