# tsv/example.tsv — handcrafted IANA TSV fixture

Generator: `../../generators/w0-fixtures/make_tsv.py`.
Verifier: `../../generators/w0-fixtures/verify_tsv.py`.

287 bytes, LF line endings, trailing newline present, 6 lines total (1 header + 5 data rows), 5 columns throughout.

## Exact content

```
id	name	qty	unit_price	note
1	Oak Panel	12	18.50	in stock
2	Steel Bracket L-90	48	2.05	backordered
3	Glass Pane 4mm	6	44.99	fragile;handle with care
4	Cable Tie 200mm	500	0.03	bag of 100 -> qty is bags
5	Weathering Test\tSample	1	0.00	literal backslash-t, NOT a real tab -- see below
```

(Tabs shown above are real `\t` (0x09) column separators; the `\t` visible *inside* the row-5 `name` field is the two literal characters backslash + `t`, not a tab byte.)

## The "no quoting" edge case (IANA TSV, RFC-less spec)

Per the [IANA TSV media type registration](https://www.iana.org/assignments/media-types/text/tab-separated-values), TSV has **no quoting or escaping mechanism**:

- A field **cannot** contain a literal tab (0x09) or newline (0x0A/0x0D) character — doing so would be indistinguishable from a column/row boundary. There is no way to escape it (unlike CSV's `"..."` quoting).
- Any producer that needs to represent a tab or newline *inside* a value must pre-process it out-of-band (e.g. replace with a space, or use a textual escape sequence like `\t`/`\n` that the *consumer* must know to un-escape — that convention is **not part of the TSV format itself**, it's application-specific).
- Row 5's `name` field (`Weathering Test\tSample`) demonstrates this: it contains the two ASCII characters `\` and `t` (an application-level escape convention some tools use), **not** a real tab byte. If it *did* contain a real tab byte, the row would parse as 6 columns instead of 5, silently corrupting the table — there is no quoting to prevent that.
- This fixture's own generator honors that constraint: every field is guaranteed free of literal `\t`/`\n`/`\r` bytes, so the file round-trips through a naive `line.split('\t')` parser with zero ambiguity.

## Verification performed

`verify_tsv.py`:
1. Confirms no `\r` bytes anywhere in the file (pure LF).
2. Splits on `\n`, then each line on `\t`; confirms **every** row has exactly 5 columns (a single inconsistent row would mean a stray tab/newline leaked into a field).
3. **Byte-exact round-trip**: rejoins the parsed rows with `\t`/`\n` and confirms the result is byte-identical to the original file (proves no information was lost or altered by the naive split/parse, i.e. the format really is unambiguous for well-formed input).
4. Confirms `trailing_newline=True`.

→ **all assertions passed** ("Byte-exact split/rejoin round-trip confirmed.").
