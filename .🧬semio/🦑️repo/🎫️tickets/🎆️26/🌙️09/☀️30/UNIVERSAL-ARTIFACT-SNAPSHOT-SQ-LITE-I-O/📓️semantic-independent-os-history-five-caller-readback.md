# OS History Five Caller Readback

Actual source already concurrently repaired before this audit; no input mutation made. Durable group replaces unsupported HistoryPageStack::last_mut with pop/update/push of exact tail. Actual stack pop332 retains physical pages and takes last allocated slot; push318/try_push322 restores the same slot without opening a page because capacity remains unchanged. Entry order and extent therefore preserved.

Hydration obtains actual ArtifactHistoryKey alongside matched edit via find_near_key and passes it to both push_applied_edit/push_redo_edit. Config indexed edit obtains key_at(position), preserving the current visible slot generation. Actual VCS key_at1112 and find_near_key1135 return genuine canonical ArtifactHistoryKey, not positional placeholder. Store APIs15828/15851 require that key and retain it in revision records. No additional static blocker found; Root actual rerun supplies compiler/runtime validation.
