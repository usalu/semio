"""Method names that std/core types (and common deps) define; a `.name(` call with such a name is not resolved by name alone."""
STD = set("""
new len is_empty iter iter_mut into_iter push pop push_back push_front pop_back pop_front insert remove get get_mut get_or_insert get_or_insert_with
contains contains_key clear extend retain drain take replace map and_then or_else unwrap_or unwrap_or_else unwrap_or_default ok_or ok_or_else is_some is_none is_ok is_err
as_ref as_mut as_str as_slice as_bytes as_deref as_deref_mut to_string to_owned to_vec clone cloned copied collect filter filter_map flat_map enumerate zip rev skip step_by chain
fold sum product min max min_by max_by min_by_key max_by_key any all find position rposition last first sort sort_by sort_by_key sort_unstable sort_unstable_by sort_unstable_by_key dedup dedup_by_key
join split splitn rsplit split_once trim trim_start trim_end trim_matches starts_with ends_with chars bytes lines parse abs sqrt powi powf floor ceil round trunc fract clamp signum
sin cos tan atan atan2 asin acos hypot exp ln log10 log2 to_degrees to_radians mul_add copysign is_nan is_finite is_infinite rem_euclid div_euclid
entry or_insert or_insert_with or_default and_modify keys values values_mut borrow borrow_mut set with_capacity reserve truncate swap splice extend_from_slice windows chunks chunks_exact concat
repeat into from try_from try_into eq ne cmp partial_cmp lt le gt ge fmt hash default lock unwrap expect ok err map_err unwrap_err expect_err
to_le_bytes from_le_bytes to_be_bytes from_be_bytes to_ne_bytes from_ne_bytes checked_add checked_sub checked_mul checked_div saturating_add saturating_sub saturating_mul wrapping_add wrapping_sub wrapping_mul
overflowing_add leading_zeros trailing_zeros count_ones pow rotate_left rotate_right is_power_of_two next_power_of_two
next peek skip_while take_while map_while inspect for_each count nth by_ref cycle sum rev flatten fuse scan partition unzip peekable last
get_unchecked get_unchecked_mut split_at split_at_mut split_first split_last iter_mut chunk_by first_mut last_mut binary_search binary_search_by binary_search_by_key
swap_remove resize resize_with fill append split_off retain_mut drain_filter extract_if capacity shrink_to_fit shrink_to leak into_boxed_slice into_vec
is_char_boundary char_indices to_uppercase to_lowercase to_ascii_uppercase to_ascii_lowercase eq_ignore_ascii_case is_ascii is_alphabetic is_alphanumeric is_numeric is_whitespace is_digit to_digit
replace replacen strip_prefix strip_suffix rfind matches match_indices bytes
insert_str push_str remove_entry get_key_value first_key_value last_key_value pop_first pop_last range range_mut
union intersection difference symmetric_difference is_subset is_superset is_disjoint
load store fetch_add fetch_sub compare_exchange swap_cell
replace_with take_cell get_cell update
poll wake wake_by_ref waker
read write read_to_end read_to_string write_all flush
as_ptr as_mut_ptr add offset
len_utf8 encode_utf8
is_positive is_negative
write_str write_fmt write_char
then then_some
filter_map unwrap_unchecked
transpose zip_with xor or and
iter_ok
min_max
dot cross length normalize lerp
type_id downcast
saturating_add
sub add mul div neg rem
bitand bitor bitxor shl shr not
index index_mut deref deref_mut drop
as_any
build
finish
field
debug_struct
from_str
value
""".split())
