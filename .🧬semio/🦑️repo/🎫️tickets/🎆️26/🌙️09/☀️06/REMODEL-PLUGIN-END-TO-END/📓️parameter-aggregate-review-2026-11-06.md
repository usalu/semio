# Parameter aggregate test review

The unmounted aggregate test retains actual original, payload and diff sources until the preparation closes, then retires the sources. Cancellation loops hardcode 0..100 rather than measuring actual successful turn counts; coverage of every frontier is not established if a path exceeds that bound. Decision and inverse denied turns currently test only maximum_items, not copy/capacity/depth/work shortfalls. Browser agent notified. No compilation or execution.
