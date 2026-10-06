impl SetLabel{
    /// 🏷️ Streams this authored mutation's original native label into its bound retained text owner.
    pub(crate) fn label_into(&self,_terminology:semio_framework_ui_locale::Terminology,locale:semio_framework_ui_locale::Locale,output:&mut dyn semio_framework_value::paged_text::TextEncodingOutput)->Result<(),semio_framework_value::ValueError>{
        match locale{
            semio_framework_ui_locale::Locale::En=>semio_framework_value::paged_text::write_encoding_format(output,format_args!("Set label to {}",self.value)),
            semio_framework_ui_locale::Locale::De=>semio_framework_value::paged_text::write_encoding_format(output,format_args!("Beschriftung auf {} setzen",self.value)),
        }
    }
}
