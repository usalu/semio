    #[test]
    fn child_complete_actual_authored_typed_source_event_observer_matches_system_backing(){
        let(mut bytes,allocated)=semio_framework_trace::observe_heap_allocations_on_this_thread(||vec![b'x';8194]);assert_eq!((allocated.requested_bytes,allocated.released_bytes),(8194,0));assert!(!allocated.overflowed);
        let(_,shortened)=semio_framework_trace::observe_heap_allocations_on_this_thread(||bytes.truncate(4096));assert_eq!((shortened.requested_bytes,shortened.released_bytes),(0,0));assert_eq!(bytes.capacity(),8194);
        let(_,resized)=semio_framework_trace::observe_heap_allocations_on_this_thread(||bytes.shrink_to_fit());assert_eq!(bytes.capacity(),4096);assert_eq!((resized.requested_bytes,resized.released_bytes,resized.largest_release_bytes),(4096,8194,8194));assert!(!resized.overflowed);
        let(_,released)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(bytes));assert_eq!((released.requested_bytes,released.released_bytes,released.largest_release_bytes),(0,4096,4096));
        let(pages,allocated)=semio_framework_trace::observe_heap_allocations_on_this_thread(||(vec![0u8;4096],vec![0u8;4096]));assert_eq!((allocated.requested_bytes,allocated.released_bytes),(8192,0));let(inner,outer)=semio_framework_trace::observe_heap_allocations_on_this_thread(||semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(pages)).1);assert_eq!(inner,outer);assert_eq!((outer.requested_bytes,outer.released_bytes,outer.largest_release_bytes),(0,8192,4096));assert!(!outer.overflowed);
        eprintln!("[DEBUG] actual system truncate releases0; realloc8194-to4096 records original8194 release+new4096 request; nested two4096 releases total8192 with largest4096");
    }
    #[test]
    fn child_complete_actual_authored_typed_source_physical8194_grant4096(){
        use semio_framework_value::retirement::{RetireOwned,RetirementStep};
        let fixture:serde_json::Value=serde_json::from_str(include_str!("@TYPED_SOURCE_PHYSICAL_FIXTURE@" )).unwrap();assert_eq!(fixture["textBytes"],8194);assert_eq!(fixture["maximumItems"],1);assert_eq!(fixture["maximumBytes"],4096);
        let mutation=crate::test_app_mutation_fixture::SetLabel{value:"x".repeat(8194)};let pointer=mutation.value.as_ptr();let backing=mutation.value.capacity();assert_eq!(backing,8194);
        let ((mut cursor,source_pointer),construction)=semio_framework_trace::observe_heap_allocations_on_this_thread(||{let source_pointer=mutation.value.as_ptr();(mutation.value.retirement(),source_pointer)});assert_eq!(pointer,source_pointer);assert_eq!(construction.released_bytes,0);assert!(!construction.overflowed);
        let (step,zero)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.close_step(0));assert!(matches!(step,RetirementStep::BudgetExhausted));assert_eq!((zero.requested_bytes,zero.released_bytes),(0,0));assert!(!cursor.terminal_is_empty());
        let mut physical=0;let mut complete=false;
        for turn in 0..8194+128{
            let (step,observed)=semio_framework_trace::observe_heap_allocations_on_this_thread(||cursor.close_step(4096));eprintln!("[DEBUG] actual typed String source retirement turn={turn} requested={} released={} largestRelease={}",observed.requested_bytes,observed.released_bytes,observed.largest_release_bytes);assert!(!observed.overflowed);assert_eq!(observed.requested_bytes,0);assert!(observed.largest_release_bytes<=4096);physical+=observed.released_bytes;
            match step{RetirementStep::Bytes(reported)=>{assert_eq!(reported,observed.released_bytes,"logical truncation cannot count as physical source release");assert!(reported<=4096)},RetirementStep::Complete=>{assert!(cursor.terminal_is_empty());complete=true;break},RetirementStep::BudgetExhausted=>{},RetirementStep::Child(_)=>panic!("actual contiguous String unexpectedly produced another source owner")}
        }
        assert!(complete,"original typed8194 source did not close under original4096 grant");assert_eq!(physical,backing);let (_,scaffold)=semio_framework_trace::observe_heap_allocations_on_this_thread(||drop(cursor));assert!(scaffold.largest_release_bytes<=4096);assert_eq!(scaffold.requested_bytes,0);assert!(scaffold.released_bytes<backing);assert!(!scaffold.overflowed);assert_eq!(physical+scaffold.released_bytes,backing+construction.requested_bytes,"every original and constructed allocation closes; copied backing cannot hide a leaked original");
    }
