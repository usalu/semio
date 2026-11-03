from ph import *
p="semio-framework-pack-record-retained"
D="🛬️decode/📦️document/🦀️.rs"
E(p,D,'=self.chunks else{unreachable!()};','=self.chunks else{return Err(invalid("retained document chunk state is not reading"))};')
E(p,D,'let manifest=self.verified.as_ref().unwrap().manifest;','let manifest=self.verified.as_ref().ok_or_else(||invalid("missing verified document manifest"))?.manifest;')
RC="🫴️receiving/📦️document/🦀️.rs"
E(p,RC,"fn driver(&self)->Option<&RetainedDocumentDriver>{self.initialized.then(||unsafe{self.storage.as_ref().unwrap().assume_init_ref()})}","fn driver(&self)->Option<&RetainedDocumentDriver>{if self.initialized{self.storage.as_ref().map(|storage|unsafe{storage.assume_init_ref()})}else{None}}")
E(p,RC,".map(|driver|{self.storage.as_mut().unwrap().write(driver);self.initialized=true;None});",'.and_then(|driver|{let Some(storage)=self.storage.as_mut()else{return Err(PackRefusal::from(invalid("Document driver storage is absent")))};storage.write(driver);self.initialized=true;Ok(None)});')
E(p,RC,"let driver=self.driver_mut().unwrap();",'let Some(driver)=self.driver_mut()else{return RetainedWorkOutcome{result:Err(PackRefusal::from(invalid("Document driver is not initialized"))),work:0}};')
if __name__=="__main__":build()
