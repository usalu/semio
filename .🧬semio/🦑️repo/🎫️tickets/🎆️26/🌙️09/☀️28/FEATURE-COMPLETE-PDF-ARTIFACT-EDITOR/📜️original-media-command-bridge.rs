//! 🌉️ Binds the framework’s actual fixed command pages to one issued media operation.

use crate::{ingress::OriginalMediaLoan, receiver::OriginalMediaPage};
use semio_framework_replication_command_ingress::{CommandPageCursor, FixedCommandPage, COMMAND_PAGE_MAXIMUM_BYTES};
use semio_framework_value::ValueError;

/// 🪪️ Associates one admitted command envelope with the exact pending image edit.
pub struct OriginalMediaCommandBinding<'a> {
    pub loan: OriginalMediaLoan<'a>,
    pub owner: u64,
    pub seq: u64,
    pub kind: u8,
    pub declared: u32,
}

impl OriginalMediaCommandBinding<'_> {
    /// 📥️ Borrows a staged WIT body until the actor has accepted its complete physical copy.
    pub fn body<'page>(&'page self, cursor: &CommandPageCursor, page: &'page FixedCommandPage) -> Result<OriginalMediaPage<'page>, ValueError> {
        let pages = self.declared.div_ceil(COMMAND_PAGE_MAXIMUM_BYTES as u32);
        if self.declared == 0
            || cursor.owner != self.owner || cursor.seq != self.seq || cursor.kind != self.kind
            || cursor.generation != self.loan.generation || cursor.instance != self.loan.instance_id
            || cursor.command_index != 0 || cursor.command_count != 1 || cursor.item_count != 1
            || cursor.metadata != self.declared || cursor.page_count != pages || cursor.page_index >= pages
        {
            return Err(ValueError::invariant("Original media page changed its authorized command cursor"));
        }
        let remaining = self.declared as usize - cursor.page_index as usize * COMMAND_PAGE_MAXIMUM_BYTES;
        if page.len() != remaining.min(COMMAND_PAGE_MAXIMUM_BYTES) {
            return Err(ValueError::invariant("Original media page changed its authorized byte extent"));
        }
        Ok(OriginalMediaPage::new(self.loan, cursor.page_index, page.as_slice()))
    }
}
