# Member failure storage frontier

Normal processing gives recipient payload owner priority while spare-only storage waits for the decoder to return; close drains spare after owned work, without using the spare itself to trigger refill. Refill is selected before effects and obeys existing grant and zero-fuel checks.

Concrete gap: failure_storage_needed checks active only when its payload is nonterminal. close_factory_ticket_controlled requires a spare even for a terminal ticket shell. If the last spare was consumed by a failure and active remains terminal with no live decoder or recipient owner, demands quotes only shell release; the ticket then attempts refill under a grant lacking storage capacity and can repeat indefinitely. The frontier must include a present active ticket regardless of its payload terminal state; its shell remains a separate action. Source-only review; no runtime validation.
