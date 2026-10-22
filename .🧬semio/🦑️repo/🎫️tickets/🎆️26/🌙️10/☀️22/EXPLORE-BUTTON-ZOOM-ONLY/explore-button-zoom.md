# Explore Button Zoom Only

The layered overview zoom (the glide that shows the page behind a card) starts only while the pointer is over the card's explore control: the primary footer action, or the open chip when the whole card is the button. That is Explore on the architecture course, Open on semio-tech play, and the open chip on the mit-bestand demonstrator.

Keyboard focus anywhere in the card still reveals the page.

When an overview becomes the active level, a control the pointer is already on does not zoom. A move of 8px marks that hover as real, and leaving the control then lets the next hover zoom. A move of 48px zooms immediately if the pointer is still on the control. A level that starts interactive, with no previous inactive frame, is unchanged.
