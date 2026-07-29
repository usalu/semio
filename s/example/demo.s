programs=[ draw writer raster note ]
workflow {
  schema=s.workflow
  nodes [id:TEXT instance-id:TEXT x:NUM y:NUM width:NUM height:NUM inputs:LIST outputs:LIST] {
    node-app-draw-1 app-draw-1 40 80 160 72 [ id="app-draw-1:out:in" resource-kind="2d.drawing" direction=in ] [ id="app-draw-1:out:out" resource-kind="2d.drawing" direction=out ]
    node-app-draw-2 app-draw-2 220 80 160 72 [ id="app-draw-2:out:in" resource-kind="2d.drawing" direction=in ] [ id="app-draw-2:out:out" resource-kind="2d.drawing" direction=out ]
    node-app-writer-1 app-writer-1 420 80 160 72 [ id="app-writer-1:out:in" resource-kind=text.document direction=in ] [ id="app-writer-1:out:out" resource-kind=text.document direction=out ]
    node-app-raster-1 app-raster-1 40 200 160 72 [ id="app-raster-1:out:in" resource-kind="2d.raster" direction=in id="app-raster-1:param.param-brush-size:in" resource-kind=parameter.value direction=in ] [ id="app-raster-1:out:out" resource-kind="2d.raster" direction=out ]
    node-app-note-2 app-note-2 220 220 160 72 [ id="app-note-2:out:in" resource-kind="2d.note" direction=in ] [ id="app-note-2:out:out" resource-kind="2d.note" direction=out ]
  }
  edges [id:TEXT source-node-id:TEXT source-port-id:TEXT target-node-id:TEXT target-port-id:TEXT contract:BLOCK] {
    edge-draw-1-to-draw-2 node-app-draw-1 "app-draw-1:out:out" node-app-draw-2 "app-draw-2:out:in" {
      kind_id="2d.drawing" class=data form=value wire_kind=document wire_schema="2d.drawing"
    }
  }
}
app-instances [id:TEXT program-id:TEXT app-id:TEXT label:TEXT yields:TEXT document:BLOCK] {
  app-draw-1 draw draw "Semio Emblem" "2d.drawing" {
    document-id=doc-app-draw-1 schema=draw.document
  }
  app-draw-2 draw draw "Emblem Copy" "2d.drawing" {
    document-id=doc-app-draw-2 schema=draw.document
  }
  app-writer-1 writer writer "Jack Notes" text.document {
    document-id=doc-app-writer-1 schema=writer.document
  }
  app-raster-1 raster raster "Raster Board" "2d.raster" {
    document-id=doc-app-raster-1 schema=raster.document
  }
  app-note-2 note document "Note Board" "2d.note" {
    document-id=doc-app-note-2 schema=note.document
  }
}
parameter-bindings [parameter-id:TEXT instance-id:TEXT field-path:TEXT] {
  param-brush-size app-raster-1 "/brushSize"
}
numeric id=param-brush-size name="Brush Size" value=32 min=1 max=128 step=1
categorical id=param-quality name=Quality value=High options=[ Draft High ]
