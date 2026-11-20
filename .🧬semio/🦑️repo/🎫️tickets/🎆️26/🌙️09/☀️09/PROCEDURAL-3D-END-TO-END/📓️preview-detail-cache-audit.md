# Preview channel routing and detail cache audit

A Flow wire with an empty source port routes the full dictionary. The generation3d export bridge previously retained it as a named empty port, which selected no geometry. The bridge now treats it as all source channels and drops repeated named selectors for the same source. The targeted Rust witness passed and SQLite found the expected geometry for all three export requests in the neutral fixture. The artifact-surface suite now also checks the union used by the background tessellator.

## Detail cache

The existing viewer already requests a fresh evaluation tick when tolerance changes. The editor now does the same when a config mutation changes the effective tolerance. Each resident cache row owns its canonical mesh and the exact tolerance that produced it. Request admission checks the desired tolerance and keeps one active request per handle. The prior mesh stays available while the replacement is being received and is retired through the original paid replacement.

Graph changes pin in-flight handles until their requests settle. The editor and viewer command receiver also closes a stale donated reply through its original owner when the corresponding request is gone. It does not overwrite the current mesh or settle another request’s ledger. The registered preview fixture covers an old reply arriving with another request in flight.

The SQLite oracle passed four tests and 28 assertions over the same admission, replacement, stale reply and fault fixtures. A cached native witness compiled the live receiver and passed admission. A second cached native witness drove the actual typed reply controller and original ledger with bounded grants, including a terminal fault and a late reply. The linked preview fixtures and browser launch remain pending.

A routed extension fault now records terminal progress while the request still owns its ledger, then settles its addressed window. The rejected tolerance is suppressed until the relevant geometry is discarded or the request is cancelled. A different tolerance remains admissible, can use the last resident mesh while it runs, and does not inherit the failed detail’s diagnostic status. The malformed routing path still returns its original fault through paid command retirement.
