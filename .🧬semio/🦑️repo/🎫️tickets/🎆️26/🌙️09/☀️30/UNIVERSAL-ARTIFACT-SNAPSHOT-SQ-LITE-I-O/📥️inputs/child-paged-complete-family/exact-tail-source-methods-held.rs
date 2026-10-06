    /// 📏️ Admits the actual remaining final payload extent without reserving a full unused page.
    pub fn next_exact_capacity_allocation_bytes(&self, limit: usize) -> Result<Option<usize>, PagedListError> {
        if limit > N || limit < self.length {
            return Err(PagedListError { kind: PagedListRefusalKind::OwnershipLimit, reason: "exact final extent exceeds logical authority" });
        }
        if limit <= self.capacity { return Ok(None); }
        self.next_page_allocation_bytes_for_limit(limit).map(Some)
    }

    /// 🧱️ The source's declared final extent funds one genuine metadata or payload allocation.
    pub fn reserve_exact_capacity_one(&mut self, limit: usize, grant: usize) -> Result<PagedListProgress, PagedListAllocationError> {
        if limit > N || limit < self.length {
            return Err(PagedListAllocationError { allocated_bytes: 0, kind: PagedListRefusalKind::OwnershipLimit, reason: "exact final extent exceeds logical authority" });
        }
        if limit <= self.capacity { return Ok(PagedListProgress::default()); }
        self.reserve_page_using_for_limit::<ExactAllocation>(grant, limit)
    }
