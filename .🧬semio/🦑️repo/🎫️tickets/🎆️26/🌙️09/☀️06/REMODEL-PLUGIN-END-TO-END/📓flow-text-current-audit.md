# Current Flow text audit

Phase-zero String::try_reserve_exact quotes source length but may allocate a larger capacity before detecting any actual receipt. Existing decode construction backing uses checked Layout::array, exact alloc and Vec::from_raw_parts; an empty exact Vec can become String via from_utf8_unchecked without copying. Root notified.

Current UTF-8 copy work quote bounds the selected up-to-4096 byte copy plus boundary probes and fixed work in inspected source. New law exercises zero and short work with real Job loan, exact funded turn, source identity, heap receipts and cancellation. This is a source review only; no execution.

The indexed exact_vec draft uses matching checked layout and Vec capacity, including empty and zero-size handling, resolving the previously identified requested-capacity issue in those allocations.