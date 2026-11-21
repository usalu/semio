//! 🧾 Accounts native raw picker ingress within the existing retained kernel request queue.
use semio_framework::kernel::{CommandPageCursor, Event, FixedCommandPage, OriginalMediaControlEvent,
    OriginalMediaWireControl, COMMAND_MAXIMUM_BYTES, COMMAND_PAGE_MAXIMUM_BYTES, ORIGINAL_MEDIA_COMMAND_KIND};

pub(super) enum QueuedOriginalMedia {
    Control(OriginalMediaControlEvent),
    Page(CommandPageCursor, FixedCommandPage),
    Poll,
}

impl QueuedOriginalMedia {
    pub(super) fn admit(event: Event) -> Result<Self, Event> {
        match event {
            Event::OriginalMediaControl(control) if valid_control(&control) => Ok(Self::Control(control)),
            Event::CommandIngressPage { cursor, bytes } if cursor.kind == ORIGINAL_MEDIA_COMMAND_KIND
                && cursor.command_index == 0 && cursor.command_count == 1 && cursor.item_count == 1
                && cursor.instance != 0 && cursor.owner != 0 && cursor.seq != 0
                && cursor.metadata as usize <= COMMAND_MAXIMUM_BYTES
                && cursor.page_count == cursor.metadata.div_ceil(COMMAND_PAGE_MAXIMUM_BYTES as u32)
                && cursor.page_index < cursor.page_count
                && bytes.len() == (cursor.metadata as usize - cursor.page_index as usize * COMMAND_PAGE_MAXIMUM_BYTES).min(COMMAND_PAGE_MAXIMUM_BYTES) =>
                Ok(Self::Page(cursor, bytes)),
            Event::Wake => Ok(Self::Poll),
            rejected => Err(rejected),
        }
    }

    pub(super) fn into_event(self) -> Event {
        match self {
            Self::Control(control) => Event::OriginalMediaControl(control),
            Self::Page(cursor, bytes) => Event::CommandIngressPage { cursor, bytes },
            Self::Poll => Event::Wake,
        }
    }

    pub(super) fn credits(&self) -> (usize, usize) {
        match self {
            Self::Control(control) => (0, original_control_bytes(control)),
            Self::Page(..) => (1, COMMAND_PAGE_MAXIMUM_BYTES),
            Self::Poll => (0, 0),
        }
    }
}

fn valid_control(control: &OriginalMediaControlEvent) -> bool {
    let file_valid = match &control.control {
        OriginalMediaWireControl::Begin { name, media_type } => !name.is_empty() && name.len() <= 1_020
            && !media_type.is_empty() && media_type.len() <= 255 && control.extent != 0,
        OriginalMediaWireControl::Complete => control.extent != 0,
        OriginalMediaWireControl::Cancel => true,
    };
    control.kind == ORIGINAL_MEDIA_COMMAND_KIND && control.owner != 0 && control.seq != 0
        && control.loan.instance_id != 0 && control.extent as usize <= COMMAND_MAXIMUM_BYTES
        && control.loan.operation_id.len() == 16 && control.loan.generation.len() == 16
        && control.loan.base_revision.len() == 64 && !control.loan.document_id.is_empty()
        && control.loan.document_id.len() <= 1_024 && file_valid
        && original_control_bytes(control) <= COMMAND_PAGE_MAXIMUM_BYTES
}

fn original_control_bytes(control: &OriginalMediaControlEvent) -> usize {
    let loan = &control.loan;
    loan.operation_id.capacity() + loan.generation.capacity() + loan.base_revision.capacity()
        + loan.document_id.capacity() + match &control.control {
            OriginalMediaWireControl::Begin { name, media_type } => name.capacity() + media_type.capacity(),
            _ => 0,
        }
}
