# UI Textarea Multi-Consumer Retention Audit

- Component SHA-256: `228cbc4bdd3c899f28527bca6c3fdfc69face9945177a05ec83fc9947e81c7f1`, clean.
- Story SHA-256: `dc818f9ba89ce2d9871732331c34fc40ce08b67e8d8c92b2d56916aa8f592703`, clean.
- React index at audit time: `f4415689af8fadf41714bde7b4bc7181169804a7b878ee25411791ec8d5abf59`.

Textarea has multiple independent active production consumers: framework Tree, framework IconSelector, protected renderer TextEditor, and protected renderer Interpreter. The package barrel, renderer package index, story, and owner-local tests do not change the qualifying count.

Decision: retain Textarea at the framework UI owner. It exceeds the two-independent-production-consumer threshold and its current owner is the lowest common owner across framework and product terminals. No source edit follows.
