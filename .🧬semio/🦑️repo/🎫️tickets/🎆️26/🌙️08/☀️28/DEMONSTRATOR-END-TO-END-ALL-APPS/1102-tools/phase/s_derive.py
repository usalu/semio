from ph import *
ex="semio-framework-value-derive-expansion"
E(ex,"⚙️expansion/🦀️.rs",'''fn container_equality_path(container: &ContainerAttrs) -> syn::Path {
    let path = container.equality_path.as_deref().or(container.crate_path.as_deref()).unwrap_or("::semio_framework_value");
    syn::parse_str(path).expect("valid #[value(equality = \\"...\\")] path")
}''','''fn container_equality_path(container: &ContainerAttrs) -> syn::Result<syn::Path> {
    let path = container.equality_path.as_deref().or(container.crate_path.as_deref()).unwrap_or("::semio_framework_value");
    syn::parse_str(path).map_err(|error| syn::Error::new(error.span(), "#[value(equality = \\"...\\")] requires a valid path"))
}''')
E(ex,"⚙️expansion/🦀️.rs","pub fn declared_equality_crate(input:&DeriveInput)->syn::Result<syn::Path>{Ok(container_equality_path(&parse_container_attrs(&input.attrs)?))}","pub fn declared_equality_crate(input:&DeriveInput)->syn::Result<syn::Path>{container_equality_path(&parse_container_attrs(&input.attrs)?)}")
d="semio-framework-value-derive"
R="🧬️retained-clone/🦀️.rs"
E(d,R,'fn construct_fields(name: &syn::Ident, fields: &Fields, values: &[syn::Ident], variant: Option<&syn::Ident>) -> TokenStream {','fn construct_fields(value_crate: &syn::Path, name: &syn::Ident, fields: &Fields, values: &[syn::Ident], variant: Option<&syn::Ident>) -> TokenStream {')
E(d,R,'quote!(self.#value.take().expect("retained field owner"))','quote!(self.#value.take().ok_or_else(||#value_crate::ValueError::literal(#value_crate::ValueRefusalKind::InvariantViolated,"retained field owner is absent"))?)')

E(d,R,'let construct = construct_fields(name, fields,','let construct = construct_fields(&value_crate, name, fields,')
E(d,R,'let construct=construct_fields(name,&variant.fields,','let construct=construct_fields(&value_crate,name,&variant.fields,')
E(d,R,'let names = named.named.iter().map(|field| field.ident.as_ref().expect("named field"));\n            quote!(#prefix { #(#names: #owners),* })','let names = named.named.iter().filter_map(|field| field.ident.as_ref());\n            quote!(#prefix { #(#names: #owners),* })')
if __name__=="__main__":build()
