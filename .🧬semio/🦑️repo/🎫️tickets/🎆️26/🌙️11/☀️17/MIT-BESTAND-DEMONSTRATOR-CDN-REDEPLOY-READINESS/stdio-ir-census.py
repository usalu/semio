from collections import Counter
import heapq
import re
import sys

fams = {name: (name,) for name in ('with_loan','with_original_effect_at','step_decode_forward','with_construction_recipient','with_work_recipient','dispatch_native_original','admit_original_construction', 'with_native_receiving_quoted', 'run_producer', 'receive_original_effect_dyn', 'drive_original_effect_dyn', 'finish_effect_retained_mode', 'step_original', 'step_reply', 'step_with', 'admit_retirement_cursor', 'NativeReceiverBackingClose', 'NativeReceivingEffect', 'native_close_settle', 'original_work', 'admit_original_session', 'from_value', 'clone_owned_into', 'retirement_cursor', 'advance_inner', 'retire_slot', 'drop_in_place', 'original_checkout', 'OriginalCheckpointCheckout', 'OriginalTaggedPackValueReceiving', 'subtle_xsd', 'acknowledge', 'register_family')}
fams.update({'retirement turn with_effect': ('OriginalRetirementTurn',), 'OriginalDecodeState': ('OriginalDecodeState',), 'with_effect': ('11with_effect',), 'NativeEncoderWork': ('NativeEncoderWork',), 'run_original': ('run_original',)})
mixed = Counter(); lines = Counter(); sample = []
fn = None
size = 0
all_lines = function_count = 0
for line in sys.stdin:
    all_lines += 1
    if line.startswith('define '):
        fn=line.partition('@')[2].partition('(')[0]
        size=1
    elif fn is not None:
        size += 1
        if line.startswith('}'):
            function_count += 1
            found=[key for key, needles in fams.items() if any(needle in fn for needle in needles)]
            if all(needle in fn for needle in ('with_loan', 'with_original_effect_at')):
                found += ['with_loan under with_original_effect_at']
            if size == 983 and all(needle in fn for needle in ('with_loan', 'step_decode_forward')):
                found += ['983-line hydration Work loan']
            for key in found or ['none of tracked families']:
                mixed[key] += 1
                lines[key] += size
            if len(sample)<18: heapq.heappush(sample,(size,fn))
            elif size>sample[0][0]: heapq.heapreplace(sample,(size,fn))
            fn=None
print("[DEBUG]",sys.argv[1],all_lines,function_count)
for key,count in lines.most_common(): print(key,mixed[key],count)
for size,fn in sorted(sample,reverse=True): print(size,fn)
