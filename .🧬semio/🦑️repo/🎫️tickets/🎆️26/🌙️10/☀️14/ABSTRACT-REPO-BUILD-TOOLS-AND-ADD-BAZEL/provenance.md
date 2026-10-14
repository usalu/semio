# Process Provenance

Read-only process environment inspection emitted only ticket-membership and thread-equality booleans. Raw environment, process wire, tokens and thread identifiers were not printed or retained. A matching artifact directory alone is not sufficient authority to stop a process.

```json
[
  {
    "pid": 23688,
    "present": true,
    "artifactDirectoryBelongsToTicket": null,
    "ticketDirectoryBelongsToTicket": null,
    "threadIdentityComparable": false,
    "sameThread": null
  },
  {
    "pid": 14260,
    "present": false,
    "artifactDirectoryBelongsToTicket": null,
    "ticketDirectoryBelongsToTicket": null,
    "threadIdentityComparable": false,
    "sameThread": null
  },
  {
    "pid": 18838,
    "present": false,
    "artifactDirectoryBelongsToTicket": null,
    "ticketDirectoryBelongsToTicket": null,
    "threadIdentityComparable": false,
    "sameThread": null
  },
  {
    "pid": 22611,
    "present": false,
    "artifactDirectoryBelongsToTicket": null,
    "ticketDirectoryBelongsToTicket": null,
    "threadIdentityComparable": false,
    "sameThread": null
  }
]
```
