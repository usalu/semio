//! 🔗️ Captures both borrowed polygon inputs through the existing polygon parser.

use super::*;
use semio_framework_3d::mesh::{MeshAuthoredDifference,MeshAuthoredIntersection,MeshAuthoredUnion,PolygonCombine};
use semio_framework_mesh_engine::{PolygonMeshSource,io::text::PolygonSourceGrant};

#[derive(semio_framework_value::RetireOwned)]
#[value(send_only)]
enum PairJob {
    Combine(PolygonCombine),
    Intersection(MeshAuthoredIntersection),
    Difference(MeshAuthoredDifference),
    Union(MeshAuthoredUnion),
}

#[derive(semio_framework_value::RetireOwned)]
#[value(send_only)]
pub struct CombineSource {
    left_parser:PolygonSourcePreparation,
    right_parser:PolygonSourcePreparation,
    left:Option<PolygonMeshSource>,
    right:Option<PolygonMeshSource>,
    job:Option<PairJob>,
    operation:&'static str,
    side:usize,
    receipt:[usize;4],
}

impl CombineSource {
    pub(super) fn new(operation:&'static str)->Self {
        Self{left_parser:PolygonSourcePreparation::new(),right_parser:PolygonSourcePreparation::new(),left:None,right:None,job:None,operation,side:0,receipt:[0;4]}
    }

    pub(super) fn phase(&self)->&'static str {
        match &self.job{
            Some(PairJob::Combine(job))=>job.phase(),
            Some(PairJob::Intersection(_))=>"mesh-intersect",
            Some(PairJob::Difference(_))=>"mesh-difference",
            Some(PairJob::Union(_))=>"mesh-union",
            None if self.side==0=>"mesh-combine-left",
            None=>"mesh-combine-right",
        }
    }

    pub(super) fn demand(&self,inputs:[&str;2],copy:usize)->Result<RetirementDemand,ValueError> {
        match &self.job{
            Some(PairJob::Combine(job))=>job.demand(copy),
            Some(PairJob::Intersection(job))=>job.demand(copy),
            Some(PairJob::Difference(job))=>job.demand(copy),
            Some(PairJob::Union(job))=>job.demand(copy),
            None=>{
                let parser=if self.side==0{&self.left_parser}else{&self.right_parser};
                let grant=parser.cold_grant(inputs[self.side],1,copy.max(1)).map_err(|_|ValueError::literal(semio_framework_value::ValueRefusalKind::InvalidValue,"unable to quote the second mesh source"))?.retirement;
                Ok(RetirementDemand{copy_bytes:grant.maximum_copy_bytes.max(1),capacity_bytes:grant.maximum_capacity_bytes,release_bytes:grant.maximum_release_bytes,depth:grant.maximum_depth})
            },
        }
    }

    pub(super) fn take_progress(&mut self)->RetainedCloneProgress {
        let [copied_items,copied_bytes,retained_capacity_bytes,released_bytes]=std::mem::take(&mut self.receipt);
        RetainedCloneProgress{copied_items,copied_bytes,retained_capacity_bytes,released_bytes}
    }

    fn post(&mut self,p:RetainedCloneProgress) {self.receipt=[p.copied_items,p.copied_bytes,p.retained_capacity_bytes,p.released_bytes];}

    pub(super) fn step(&mut self,inputs:[&str;2],effect:&mut RetainedAllocationEffect<'_>)->Result<Option<PolygonMeshSource>,EvalError> {
        self.receipt=[0;4];
        if let Some(job)=&mut self.job{
            let (result,progress)=match job{
                PairJob::Combine(job)=>(job.step(effect),job.take_progress()),
                PairJob::Intersection(job)=>(job.step(effect),job.take_progress()),
                PairJob::Difference(job)=>(job.step(effect),job.take_progress()),
                PairJob::Union(job)=>(job.step(effect),job.take_progress()),
            };
            self.post(progress);
            return result.map_err(EvalError::Retained);
        }
        let grant=effect.grant();
        let parse=if self.side==0{&mut self.left_parser}else{&mut self.right_parser};
        let result=parse.step(inputs[self.side],PolygonSourceGrant{maximum_units:1,maximum_projection_bytes:grant.maximum_copy_bytes,retirement:grant},effect).map_err(invalid);
        let progress=parse.normal_step_retained_progress();
        self.post(progress);
        if let Some(right)=result?.source {
            if self.side==0{self.left=Some(right);self.side=1;return Ok(None);}
            let left=self.left.take().expect("left polygon source retained");
            let next=match self.operation{
                "intersect"=>PairJob::Intersection(MeshAuthoredIntersection::new(left,right)),
                "difference"=>PairJob::Difference(MeshAuthoredDifference::new(left,right)),
                "union"=>PairJob::Union(MeshAuthoredUnion::new(left,right)),
                _=> match PolygonCombine::new(left,right){
                    Ok(join)=>PairJob::Combine(join),
                    Err((error,left,right))=>{self.left=Some(left);self.right=Some(right);return Err(EvalError::Retained(error));}
                },
            };
            self.job=Some(next);
        }
        Ok(None)
    }
}
