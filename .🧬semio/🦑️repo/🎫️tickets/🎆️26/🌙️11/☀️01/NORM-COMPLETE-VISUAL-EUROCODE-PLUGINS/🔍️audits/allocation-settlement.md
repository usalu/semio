

The sealed original Work receipt now exposes checked remaining units, and an allocation effect takes the minimum of that actual receipt and its local window. Nested reborrow also enforces this bound. Native gate 73255 passed the actual receipt/local-window and overrun case; the independent SQLite and schema oracle passed two casesets (nine assertions).

Sealed original handback now uses separately owned source, elected SharedOriginal, and payload destination slots. Its two defining operations require the same actual effect and stage election/unique payload work before changing those owners; payload extraction checks Arc strong and weak exclusivity. The defining production check 65650 passed and the independent schema/SQLite election oracle passed. The direct native original-owner test is authored, while old test and downstream caller migrations remain in progress.
