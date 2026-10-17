# Windows relative kernel preliminary Source review

Read-only Source inspection of the appended Windows owner found three concrete points for final qualification. No Windows/Native/protected operation was performed.

readEntry currently catches ENOENT across the entire acquired handle body/close, so it must narrow null conversion to the exact failed-open not-found status, preserving arbitrary checkpoint/body/close failures. names yields on names.length%128 only at page ends, which can miss the cadence for repeated nonmultiple pages; use threshold/bounded page counter instead.

remove proves the exact child handle before SetFileInformationByHandle, then closes it and checks only parent anchor/name absence. Unlike POSIX it has no post-effect held-child identity/link transition observation. Generic facts admit hardlinks; a final NTFS contract must specify whether handlebound POSIX disposition and name absence alone suffice, or validate raw post-disposition ID/link/deletepending facts without calling the ordinary facts reader that rejects deletepending. Schema-first hardlink/deletepending/transition faults are needed before a stronger transition claim. Native NTFS acceptance remains separate from modeled ABI tests.

Other inspected boundaries correctly retain private handle/libraries, shared descriptor preacquisition reservation, no-share-delete behavior, synchronous completion/cancellation uncertainty, strong128bit ID/volume/security/stream observations, actual Flush result, fixed relative OBJECT_ATTRIBUTES and explicit Source port limits. These observations do not replace full final fault-case/hash review or actual Windows runtime acceptance.
